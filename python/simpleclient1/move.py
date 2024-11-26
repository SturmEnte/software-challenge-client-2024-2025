class Move():
    def __init__(self):
        ''' Move class
           
            available move types:
            - advance
            - fallback
            - eatsalad
            - exchangecarrots
            - missmove? or will no move request be sent in that case? TODO: test this situation
        '''
        self.cards = []
        self.type = ""
        self.parameters = {}
    
    def advance(self, distance: int):
        '''Advance forwards by given number of fields.'''
        self.type = "advance"
        self.parameters["distance"] = distance
    
    def fallback(self):
        '''Fallback to last hedgehog field.'''
        self.type = "fallback"
    
    def eat_salad(self):
        '''Eat a salad. Only allowed (and forced to) when on salad field. This move can't be used multiple times in a row.'''
        self.type = "eatsalad"
    
    def exchange_carrots(self, amount: int):
        '''Deposit or receive the given amount of carrots. To deposit the amount has to be negative.'''
        self.type = "exchangecarrots"
        self.parameters["amount"] = amount
    
    def play_card(self, type: str):
        self.cards.append(type)
    
    def __repr__(self) -> str:
        out = "----- Move -----\n"
        out += "Type: " + self.type + "\n"
        
        for parameter in self.parameters:
            out += parameter + ": " + self.parameters[parameter] + "\n"
        
        if len(self.cards) > 0:
            out += "\nCards:\n"
            for card in self.cards:
                out += card + "\n"

        return out