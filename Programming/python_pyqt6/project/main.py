import sys
import PyQt6 as qt

from PyQt6.QtWidgets import QApplication, QWidget

class main_window(QWidget):
    def __init__(self):
        super().__init__()

        self.show()

if __name__ == '__main__':
    qt = QApplication(sys.argv)
    app = main_window()
    sys.exit(qt.exec())
