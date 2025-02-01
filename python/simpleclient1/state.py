from compute import get_hedgehog_field, get_needed_carrots

class State():
    def __init__(self, team, turn, start_team, board, players):
        self.team = team
        self.turn = turn
        self.start_team = start_team
        self.board = board
        self.game_over = False
        self.winner = None

        if self.team == "ONE":
            self.opponent_team = "TWO"
        else:
            self.opponent_team = "ONE"

        if players[0].team == self.team:
            self.player = players[0]
            self.opponent = players[1]
        else:
            self.player = players[1]
            self.opponent = players[0]
    
    def set_data(self, turn, players):
        '''Updates all variable state values'''

        self.turn = turn
        
        if players[0].team == self.team:
            self.player = players[0]
            self.opponent = players[1]
        else:
            self.player = players[1]
            self.opponent = players[0]
    
    def market_or_hare_field(self, move, player, other_player):
        new_field = self.board.getField(player.position)

        # if the accessed field is a hare field (card has to be played)
        if new_field.type == "HARE":
            self.play_card(move, player, other_player)
        
        # if the accessed field is a market field (card has to be bought and +10 carrots)
        elif new_field.type == "MARKET":
            self.buy_card(move, player)
    
    def buy_card(self, move, player):
        card = move.cards.pop(1)

        player.carrots -= 10
        player.cards.append(card)
    
    def play_card(self, move, player, other_player):
        card = move.cards.pop(1)

        if card == "EAT_SALAD":
            player.salads -= 1
            
            if other_player.position > player.position:
                player.carrots += 30
            else:
                player.carrots += 10
        
        elif card == "HURRY_AHEAD":
            player.position = other_player.position + 1
        
        elif card == "FALL_BACK":
            player.position = other_player.position - 1
        
        elif card == "SWAP_CARROTS":
            carrots = player.carrots
            player.carrots = other_player.carrots
            other_player.carrots = carrots
        
        # check for market or hare field
        if card == "HURRY_AHEAD" or card == "FALL_BACK":
            self.market_or_hare_field(move, player, other_player)
    
    def apply_move(self, move, own_player=True):
        '''Applies a move to the board using the selected team.'''

        self.turn += 1

        # select player to apply move for
        if own_player:
            player = self.player
            other_player = self.opponent
        else:
            player = self.opponent
            other_player = self.player

        current_field = self.board.getField(player.position)

        # add carrots, if a contition for a position 1 / 2 field is met
        if current_field.type == "POSITION_1" and other_player.position < player.position:
            player.carrots += 10
        elif current_field.type == "POSITION_2" and other_player.position > player.position:
            player.carrots += 30

        # apply the given move
        if move.type == "advance":
            distance = move.parameters["distance"]
            player.position += distance
            player.carrots -= get_needed_carrots(distance)

            # the same code needs to be executed if a hare / market field is accessed using a card instead of an advance move
            self.market_or_hare_field(move, player, other_player)                
        
        elif move.type == "fallback":
            hedgehog_field, distance = get_hedgehog_field(self, player.position)
            player.position = hedgehog_field.index
            player.carrots += distance * 10
        
        elif move.type == "exchangecarrots":
            player.carrots += move.parameters["amount"]
        
        elif move.type == "eatsalad":
            player.salads -= 1
            
            if other_player.position > player.position:
                player.carrots += 30
            else:
                player.carrots += 10
        else:
            print("Unknown Move:", move.type)
        
        # check if game over condition is met
        if self.turn == 60 or player.position == 64 or other_player.position == 64:
            self.game_over = True
        
            # if the game has reached the turn limit
            if self.turn == 60:
                if self.player.position > self.opponent.position:
                    self.winner = self.team
                else:
                    self.winner = self.opponent_team
            
            # if we reached the goal
            elif self.player.position == 64:
                self.winner = self.team

            # if the opponent reached the goal
            else:
                self.winner = self.opponent_team
            
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
    
    def print_player(self, print_own_player=True):
        player = None

        if print_own_player:
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