class State():
    def __init__(self, team, turn, start_team, board, players):
        self.team = team
        self.turn = turn
        self.start_team = start_team
        self.board = board

        if players[0].team == self.team:
            self.player = players[0]
            self.opponent = players[1]
        else:
            self.player = players[1]
            self.opponent = players[0]
    
    def set_data(self, turn, board, players):
        '''Updates all variable state values'''
        self.turn = turn
        self.board = board
        
        if players[0].team == self.team:
            self.player = players[0]
            self.opponent = players[1]
        else:
            self.player = players[1]
            self.opponent = players[0]
    
    def print_board(self):
        
        out1 = "----------Board-----------\n"
        out = ""
        out2 = ""

        for field in self.board.board:
            out1 += str(field.index).ljust(2, ' ')
            if self.player.position == field.index:
                out2 += str(self.player)
            elif self.opponent.position == field.index:
                out2 += str(self.opponent)
            else:
                out2 += "  "
            out += str(field)
            out += " "
            out1 += " "
            out2 += " "
        
        print(out1 + "\n" + out + "\n" + out2)
    
    def print_player(self, printOwnPlayer=True):
        player = None

        if printOwnPlayer:
            player = self.player
        else:
            player = self.opponent

        out = f"""
----------Player----------
Team:       {player.team}
Position:   {player.position}
Salads:     {player.salads}
Carrots:    {player.carrots}
Cards:
"""

        print(out)

    def print_state(self):
        print("---------------State---------------\n")
        self.print_board()
        self.print_player()
        self.print_player(False)
        print("-----------------------------------")