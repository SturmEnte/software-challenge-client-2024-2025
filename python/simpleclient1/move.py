class Move():
    def __init__(self):
        self.actions = []
        self.index = 0
    
    def advance(self, distance: int):
        '''Advance in the current direction by given number of fields.'''
        self.actions.append(f'<advance distance="{distance}" />')
    
    def undo(self):
        '''removes the last action from the move'''
        self.actions.pop(-1)
    
    def __repr__(self) -> str:
        out = "<actions>\n"
        for action in self.actions:
            out += "    " + action + "\n"
        out += "</actions>"
        return out