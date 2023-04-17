from examples.pygui import app

class Application(object):
    def __init__(self):
        self.inner = app.Application()

    def extra_window(self):
        app.gui.begin("Python Window")
        app.gui.text("Hello from Python!")
        app.gui.end()

    def run(self):
        while self.inner.prepare_frame():
            self.extra_window()
            self.inner.render_frame()

if __name__ == '__main__':
    a = Application()
    a.run()
