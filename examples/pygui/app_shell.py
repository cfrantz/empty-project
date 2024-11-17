import argparse
import IPython
from threading import Thread
from examples.pygui import libapp
from examples.pygui.libapp import gui

class Application(object):
    @staticmethod
    def runner():
        a = Application()
        a.run()
        raise SystemExit

    def __init__(self):
        self.inner = libapp.Application()
        self.inner.set_scale(2.0)
        self.show_style_editor = False

    def extra_window(self, ui):
        gui.begin("Python Window")
        gui.text("Hello from Python!")
        self.inner.rust_fragment(ui)
        gui.end()

    def style_editor(self):
        if not self.show_style_editor:
            return
        
        (window, self.show_style_editor) = gui.begin("Style Editor", self.show_style_editor)
        if window:
            gui.show_style_editor(gui.get_style())
        gui.end()

    def menu_bar(self):
        gui.begin_main_menu_bar()

        if gui.begin_menu("File"):
            gui.menu_item("New")
            gui.menu_item("Open")
            gui.menu_item("Save")
            gui.end_menu()

        if gui.begin_menu("Edit"):
            gui.menu_item("Cut")
            gui.menu_item("Copy")
            gui.menu_item("Paste")
            if gui.menu_item("Style Preferences"):
                self.show_style_editor = True
            gui.end_menu()

        gui.end_main_menu_bar()
        pass

    def run(self):
        while ui := self.inner.prepare_frame():
            self.menu_bar()
            self.style_editor()
            self.inner.rust_window(ui)
            self.extra_window(ui)
            self.inner.render_frame()

if __name__ == '__main__':
    p = argparse.ArgumentParser(prog="app", description = "Sample App")
    p.add_argument("--interactive", "-i", action="store_true", help="Start an interactive Python shell")
    args = p.parse_args()

    if args.interactive:
        thread = Thread(target=Application.runner, daemon=True)
        thread.start()
        IPython.embed()
    else:
        Application.runner()
