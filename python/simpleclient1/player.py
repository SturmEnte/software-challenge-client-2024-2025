try:
    from colorama import Back
    RED = Back.RED
    RESET = Back.RESET
except:
    RED = ""
    RESET = ""

class Player():
    def __init__(self, team, position, salads, carrots):
        ''' Player class

            available teams:
            - ONE
            - TWO

            available positions:
            0 - 64

            available salads:
            0 - 5

            available carrots:
            0 - ?
        
        '''
        self.team = team
        self.position = position
        self.salads = salads
        self.carrots = carrots
    
    def set_position(self, x):
        self.position = x
    
    def get_position(self):
        return self.position
    
    def __str__(self):
        out = ""

        if self.team == "ONE":
            out = RED + "S1"
        else:
            out = RED + "S2"
        
        out += RESET
        return out