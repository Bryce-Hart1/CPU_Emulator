#!/usr/bin/env python3
"""
EM Editor -- a small syntax-highlighting window for writing EM assembly programs.

Follows the grammar defined in instructions/ASM_instructions.md and matched by the
C++ assembler in Assembler/assembler.hpp:

  * '#' comments            -> green
  * instructions            -> two colors, split by role:
                                 - flow/jumps (JMP, JMPIF0, CALL, RETURN ...) one color
                                 - everything else (ADD, MOVE, LOADIMM, INT ...) another
  * registers R0-R15        -> a few simple colors:
                                 - R0            (always 0)          grey
                                 - R1 - R11      (general purpose)   cyan
                                 - R12 - R15     (FP/SP/LR/PC)       red
  * labels (label_Name and  -> gold
    bare jump targets)
  * hex immediates (01H ...) -> orange

Programs are saved as .em files, defaulting to the Assembler/emu/ folder.
Pure-stdlib (tkinter); no external dependencies.

Run:  python3 em_editor.py
"""

import os
import re
import tkinter as tk
from tkinter import filedialog, messagebox, font as tkfont


#  Language definition (kept in lock-step with Assembler/assembler.hpp)

# instruction -> byte size, mirrors new_bytes_at() in assembler.cpp
INSTR_SIZE = {}
for _m in ("NOPERATION", "HALT", "RETURN", "CLRFLAGS"):
    INSTR_SIZE[_m] = 1                                   # basic  (1 byte)
for _m in ("ADD", "SUB", "DIV", "MULTI", "OR", "AND", "!OR", "NOT"):
    INSTR_SIZE[_m] = 2                                   # arith  (2 bytes)
for _m in ("MOVE", "MOVE&CLR", "PUSH", "POP", "INT"):
    INSTR_SIZE[_m] = 2                                   # data-mov (2 bytes)
for _m in ("LOADIMM", "JMP", "JMPIF0", "JMPIF!0", "CALL", "JMPIFCRRY", "LOAD", "STORE"):
    INSTR_SIZE[_m] = 3                                   # load/jump (3 bytes)

# Control-flow mnemonics get their own color. JMPIFAULT is documented but not yet
# assembled -- coloring it helps the user, so it is included here only for display.
FLOW_INSTR = {"JMP", "JMPIF0", "JMPIF!0", "CALL", "JMPIFCRRY", "JMPIFAULT", "RETURN"}
ALL_INSTR = set(INSTR_SIZE) | {"JMPIFAULT"}
OTHER_INSTR = ALL_INSTR - FLOW_INSTR

MAX_PROGRAM_BYTES = 4095   # jump offset ceiling, see ASM_instructions.md

REG_RE = re.compile(r"^R(\d{1,2})$")
HEX_RE = re.compile(r"^[0-9A-Fa-f]+[Hh]?$")
IDENT_RE = re.compile(r"^[A-Za-z_][A-Za-z0-9_!&]*$")
TOKEN_RE = re.compile(r"\S+")
LABEL_TAG = "label_"


def is_hex_immediate(token):
    """True for things like 01H, F1H, 0B, 40H -- hex, optionally H-suffixed."""
    if not HEX_RE.match(token):
        return False
    if token[-1] in "Hh":
        return True
    # no H suffix: only treat as a number if it actually contains a digit,
    # so all-letter identifiers (labels) are not mistaken for hex.
    return any(c.isdigit() for c in token)


def classify_token(token, is_first):
    """Return the highlight tag name for a single token, or None for plain text."""
    if token.startswith(LABEL_TAG):
        return "label"                       # label_Foo definition
    if is_first:
        if token in FLOW_INSTR:
            return "flow"
        if token in OTHER_INSTR:
            return "instr"
        return None                          # unknown mnemonic -> leave default
    # operand position
    m = REG_RE.match(token)
    if m:
        n = int(m.group(1))
        if 0 <= n <= 15:
            if n == 0:
                return "reg0"
            if n <= 11:
                return "reg_gp"
            return "reg_special"
    if is_hex_immediate(token):
        return "hex"
    if IDENT_RE.match(token):
        return "labelref"                    # bare jump target
    return None


def estimate_bytes(text):
    """Estimate assembled size the way the assembler's first pass would count it."""
    total = 0
    for raw in text.split("\n"):
        line = raw.split("#", 1)[0].strip()          # drop comments
        if not line or line.startswith(LABEL_TAG):    # blank or label def -> 0 bytes
            continue
        mnemonic = line.split()[0]
        total += INSTR_SIZE.get(mnemonic, 0)
    return total


#  Color theme (dark). Green is reserved for comments

THEME = {
    "bg":          "#1e1e1e",
    "fg":          "#d4d4d4",
    "cursor":      "#e5e5e5",
    "select":      "#264f78",
    "gutter_bg":   "#252526",
    "gutter_fg":   "#858585",
    "status_bg":   "#333333",
    "status_fg":   "#cccccc",
}

TAG_COLORS = {
    "comment":     "#6A9955",   # green   -- # comments
    "flow":        "#C678DD",   # purple  -- jumps / control flow
    "instr":       "#61AFEF",   # blue    -- all other instructions
    "reg0":        "#7F848E",   # grey    -- R0 (always zero)
    "reg_gp":      "#56B6C2",   # cyan    -- R1..R11 general purpose
    "reg_special": "#E06C75",   # red     -- R12..R15 (FP/SP/LR/PC)
    "label":       "#E5C07B",   # gold    -- label_ definitions
    "labelref":    "#E5C07B",   # gold    -- bare jump targets
    "hex":         "#D19A66",   # orange  -- hex immediates
}

LEGEND = [
    ("comment",     "# comments"),
    ("instr",       "instructions"),
    ("flow",        "jumps / flow"),
    ("reg0",        "R0"),
    ("reg_gp",      "R1-R11"),
    ("reg_special", "R12-R15"),
    ("label",       "labels"),
    ("hex",         "hex values"),
]


#  Text widget that emits a <<Change>> event on edits (for gutter + highlight)


class ChangeAwareText(tk.Text):
    def __init__(self, *args, **kwargs):
        super().__init__(*args, **kwargs)
        self._orig = self._w + "_orig"
        self.tk.call("rename", self._w, self._orig)
        self.tk.createcommand(self._w, self._proxy)

    def _proxy(self, *args):
        try:
            result = self.tk.call((self._orig,) + args)
        except tk.TclError:
            return ""
        if args and (args[0] in ("insert", "replace", "delete") or
                     args[0:3] == ("mark", "set", "insert") or
                     args[0:2] == ("xview", "moveto") or args[0:2] == ("yview", "moveto") or
                     args[0:2] == ("xview", "scroll") or args[0:2] == ("yview", "scroll")):
            self.event_generate("<<Change>>", when="tail")
        return result


class LineNumbers(tk.Canvas):
    def __init__(self, master, text_widget, **kwargs):
        super().__init__(master, **kwargs)
        self.text = text_widget

    def redraw(self):
        self.delete("all")
        i = self.text.index("@0,0")
        while True:
            dline = self.text.dlineinfo(i)
            if dline is None:
                break
            y = dline[1]
            line_no = str(i).split(".")[0]
            self.create_text(self.winfo_width() - 4, y, anchor="ne",
                             text=line_no, fill=THEME["gutter_fg"],
                             font=self.text.cget("font"))
            i = self.text.index(f"{i}+1line")


#  The editor window

class EmEditor(tk.Tk):
    def __init__(self):
        super().__init__()
        self.title("EM Editor")
        self.geometry("900x640")
        self.configure(bg=THEME["bg"])

        self.emu_dir = self._find_emu_dir()
        self.current_path = None
        self._saved_snapshot = ""

        self._build_font()
        self._build_widgets()
        self._build_menu()
        self._bind_events()

        self._configure_tags()
        self._new_file(initial=True)

    # ---- setup ---------------------------------------------------------- #

    def _find_emu_dir(self):
        here = os.path.dirname(os.path.abspath(__file__))
        candidate = os.path.join(here, "Assembler", "emu")
        return candidate if os.path.isdir(candidate) else here

    def _build_font(self):
        family = "Menlo"
        available = set(tkfont.families())
        for name in ("Menlo", "Consolas", "DejaVu Sans Mono", "Courier New", "Courier"):
            if name in available:
                family = name
                break
        self.mono = tkfont.Font(family=family, size=14)
        self.mono_small = tkfont.Font(family=family, size=11)

    def _build_widgets(self):
        editor = tk.Frame(self, bg=THEME["bg"])
        editor.pack(side="top", fill="both", expand=True)

        self.text = ChangeAwareText(
            editor, wrap="none", undo=True, font=self.mono,
            bg=THEME["bg"], fg=THEME["fg"], insertbackground=THEME["cursor"],
            selectbackground=THEME["select"], borderwidth=0,
            padx=6, pady=4, tabs=self.mono.measure("    "),
        )
        self.gutter = LineNumbers(editor, self.text, width=52,
                                  bg=THEME["gutter_bg"], highlightthickness=0)

        yscroll = tk.Scrollbar(editor, orient="vertical", command=self._on_yscroll)
        xscroll = tk.Scrollbar(self, orient="horizontal", command=self.text.xview)
        self.text.configure(yscrollcommand=yscroll.set, xscrollcommand=xscroll.set)

        self.gutter.pack(side="left", fill="y")
        yscroll.pack(side="right", fill="y")
        self.text.pack(side="left", fill="both", expand=True)
        xscroll.pack(side="bottom", fill="x")

        self._build_statusbar()

    def _build_statusbar(self):
        bar = tk.Frame(self, bg=THEME["status_bg"])
        bar.pack(side="bottom", fill="x")

        self.status = tk.Label(bar, anchor="w", bg=THEME["status_bg"],
                               fg=THEME["status_fg"], font=self.mono_small, padx=8)
        self.status.pack(side="left")

        self.bytes_lbl = tk.Label(bar, anchor="e", bg=THEME["status_bg"],
                                  fg=THEME["status_fg"], font=self.mono_small, padx=8)
        self.bytes_lbl.pack(side="right")

        # color legend
        legend = tk.Frame(self, bg=THEME["gutter_bg"])
        legend.pack(side="bottom", fill="x")
        tk.Label(legend, text="  legend:", bg=THEME["gutter_bg"],
                 fg=THEME["gutter_fg"], font=self.mono_small).pack(side="left")
        for tag, label in LEGEND:
            tk.Label(legend, text=" " + label + " ", bg=THEME["gutter_bg"],
                     fg=TAG_COLORS[tag], font=self.mono_small).pack(side="left")

    def _build_menu(self):
        menubar = tk.Menu(self)

        filemenu = tk.Menu(menubar, tearoff=0)
        filemenu.add_command(label="New", accelerator="Cmd/Ctrl+N", command=self._new_file)
        filemenu.add_command(label="Open...", accelerator="Cmd/Ctrl+O", command=self._open_file)
        filemenu.add_separator()
        filemenu.add_command(label="Save", accelerator="Cmd/Ctrl+S", command=self._save_file)
        filemenu.add_command(label="Save As...", accelerator="Cmd/Ctrl+Shift+S",
                             command=self._save_file_as)
        filemenu.add_separator()
        filemenu.add_command(label="Quit", accelerator="Cmd/Ctrl+Q", command=self._on_quit)
        menubar.add_cascade(label="File", menu=filemenu)

        helpmenu = tk.Menu(menubar, tearoff=0)
        helpmenu.add_command(label="Syntax colors", command=self._show_legend)
        helpmenu.add_command(label="About", command=self._show_about)
        menubar.add_cascade(label="Help", menu=helpmenu)

        self.config(menu=menubar)

    def _bind_events(self):
        self.text.bind("<<Change>>", self._on_change)
        self.text.bind("<KeyRelease>", self._on_change)
        self.text.bind("<ButtonRelease-1>", self._update_cursor_status)
        self.text.bind("<Configure>", lambda e: self.gutter.redraw())

        for seq in ("<Command-n>", "<Control-n>"):
            self.bind(seq, lambda e: (self._new_file(), "break")[1])
        for seq in ("<Command-o>", "<Control-o>"):
            self.bind(seq, lambda e: (self._open_file(), "break")[1])
        for seq in ("<Command-s>", "<Control-s>"):
            self.bind(seq, lambda e: (self._save_file(), "break")[1])
        for seq in ("<Command-Shift-s>", "<Control-Shift-s>"):
            self.bind(seq, lambda e: (self._save_file_as(), "break")[1])
        for seq in ("<Command-q>", "<Control-q>"):
            self.bind(seq, lambda e: (self._on_quit(), "break")[1])
        self.protocol("WM_DELETE_WINDOW", self._on_quit)

    def _configure_tags(self):
        for tag, color in TAG_COLORS.items():
            self.text.tag_configure(tag, foreground=color)

    # ---- scrolling glue ------------------------------------------------- #

    def _on_yscroll(self, *args):
        self.text.yview(*args)
        self.gutter.redraw()

    # ---- highlighting --------------------------------------------------- #

    def _highlight(self):
        text = self.text
        for tag in TAG_COLORS:
            text.tag_remove(tag, "1.0", "end")

        content = text.get("1.0", "end-1c")
        for row, line in enumerate(content.split("\n"), start=1):
            hash_idx = line.find("#")
            code = line if hash_idx == -1 else line[:hash_idx]
            if hash_idx != -1:
                text.tag_add("comment", f"{row}.{hash_idx}", f"{row}.end")

            first = True
            for m in TOKEN_RE.finditer(code):
                tag = classify_token(m.group(), first)
                first = False
                if tag:
                    text.tag_add(tag, f"{row}.{m.start()}", f"{row}.{m.end()}")

    # ---- status --------------------------------------------------------- #

    def _update_cursor_status(self, _event=None):
        row, col = self.text.index("insert").split(".")
        name = os.path.basename(self.current_path) if self.current_path else "untitled"
        dirty = "" if self._is_clean() else " *"
        self.status.config(text=f"{name}{dirty}    Ln {row}, Col {int(col) + 1}")

    def _update_bytes(self):
        n = estimate_bytes(self.text.get("1.0", "end-1c"))
        warn = "  !! over limit" if n > MAX_PROGRAM_BYTES else ""
        color = "#E06C75" if n > MAX_PROGRAM_BYTES else THEME["status_fg"]
        self.bytes_lbl.config(text=f"~{n} / {MAX_PROGRAM_BYTES} bytes{warn}", fg=color)

    def _on_change(self, _event=None):
        self._highlight()
        self.gutter.redraw()
        self._update_cursor_status()
        self._update_bytes()
        self._refresh_title()

    # ---- dirty tracking ------------------------------------------------- #

    def _is_clean(self):
        return self.text.get("1.0", "end-1c") == self._saved_snapshot

    def _mark_saved(self):
        self._saved_snapshot = self.text.get("1.0", "end-1c")
        self._refresh_title()

    def _refresh_title(self):
        name = os.path.basename(self.current_path) if self.current_path else "untitled"
        star = "" if self._is_clean() else " *"
        self.title(f"EM Editor - {name}{star}")

    def _maybe_discard(self):
        """Return True if it is OK to throw away current changes."""
        if self._is_clean():
            return True
        ans = messagebox.askyesnocancel(
            "Unsaved changes", "Save changes before continuing?")
        if ans is None:
            return False
        if ans:
            return self._save_file()
        return True

    # ---- file operations ------------------------------------------------ #

    def _new_file(self, initial=False):
        if not initial and not self._maybe_discard():
            return
        self.text.delete("1.0", "end")
        self.current_path = None
        self.text.edit_reset()
        self._mark_saved()
        self._on_change()

    def _open_file(self):
        if not self._maybe_discard():
            return
        path = filedialog.askopenfilename(
            initialdir=self.emu_dir,
            filetypes=[("EM programs", "*.em"),
                       ("EM source", "*.emu"),
                       ("All files", "*.*")])
        if not path:
            return
        try:
            with open(path, "r", encoding="utf-8") as fh:
                data = fh.read()
        except OSError as exc:
            messagebox.showerror("Open failed", str(exc))
            return
        self.text.delete("1.0", "end")
        self.text.insert("1.0", data)
        self.current_path = path
        self.text.edit_reset()
        self._mark_saved()
        self._on_change()

    def _save_file(self):
        if self.current_path is None:
            return self._save_file_as()
        return self._write_to(self.current_path)

    def _save_file_as(self):
        path = filedialog.asksaveasfilename(
            initialdir=self.emu_dir,
            defaultextension=".em",
            filetypes=[("EM program", "*.em"),
                       ("EM source (assembler)", "*.emu"),
                       ("All files", "*.*")])
        if not path:
            return False
        return self._write_to(path)

    def _write_to(self, path):
        data = self.text.get("1.0", "end-1c")
        try:
            with open(path, "w", encoding="utf-8") as fh:
                fh.write(data)
        except OSError as exc:
            messagebox.showerror("Save failed", str(exc))
            return False
        self.current_path = path
        self._mark_saved()
        self._update_cursor_status()
        return True

    def _on_quit(self):
        if self._maybe_discard():
            self.destroy()

    # ---- help ----------------------------------------------------------- #

    def _show_legend(self):
        lines = [f"{label:<16}{TAG_COLORS[tag]}" for tag, label in LEGEND]
        messagebox.showinfo("Syntax colors", "\n".join(lines))

    def _show_about(self):
        messagebox.showinfo(
            "About EM Editor",
            "A syntax-highlighting editor for EM assembly programs.\n\n"
            "Grammar follows instructions/ASM_instructions.md.\n"
            "Saves .em files (default folder: Assembler/emu/).\n\n"
            "Note: the C++ assembler currently scans for .emu files -- "
            "use Save As and pick the .emu type if you want it assembled directly.")


def main():
    app = EmEditor()
    app.mainloop()


if __name__ == "__main__":
    main()
