try:
    from colorama import Back
    GREEN = Back.GREEN
    YELLOW = Back.YELLOW
    BLACK = Back.BLACK
    BLUE = Back.BLUE
    RESET = Back.RESET
    WHITE = Back.WHITE
    RED = Back.RED
except:
    GREEN = ""
    YELLOW = ""
    BLACK = ""
    BLUE = ""
    RESET = ""
    WHITE = ""
    RED = ""

class Field():
    def __init__(self, type, index):
        ''' Board class
            
            available field types:
            - START
            - CARROT
            - HARE
            - POSITION_1
            - POSITION_2
            - SALAD
            - HEDGEHOG
            - MARKET
            - GOAL

            available indecies:
            0 - 64
        '''
        self.type = type
        self.index = index
    
    def __str__(self):
        out = "XX"
        if self.type == "START":
            out = BLACK + "ST"
        elif self.type == "CARROTS":
            out = YELLOW + "CR"
        elif self.type == "HARE":
            out = WHITE + "HA"
        elif self.type == "POSITION_1":
            out = BLUE + "P1"
        elif self.type == "POSITION_2":
            out = BLUE + "P2"
        elif self.type == "SALAD":
            out = GREEN + "SL"
        elif self.type == "HEDGEHOG":
            out = WHITE + "HE"
        elif self.type == "MARKET":
            out = RED + "MK"
        elif self.type == "GOAL":
            out = BLACK + "##"
        out += RESET
        return out
