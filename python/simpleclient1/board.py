from random import choice

class Board():
    def __init__(self):
        '''Board class'''
        self.board = [] # maybe change to numpy array for efficientcy

    def append_field(self, field):
        '''appends a new field to the end of the board'''
        self.board.append(field)
    
    def set_field(self, x, field):
        '''set a field to a new value'''
        pass

    def get_field(self, x):
        '''get a fields value'''
        pass

    
    # very old
    def update(self, move, state):
        '''Updates the board using a tuple for the move: ((fromX, fromY),(toX, toY)) if startmove then (None, (toX, toY))'''
        pass
