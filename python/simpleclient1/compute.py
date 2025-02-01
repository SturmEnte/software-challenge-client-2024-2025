# TODO: manage coal when turning
# TODO: prevent ship from reaching speeds below 1

from move import Move
from time import time

TIME_LIMIT = 1.8

def get_needed_carrots(n):
    '''Rennkarte'''
    return int((n * (n + 1)) / 2)

def get_hedgehog_field(state, position):
    '''Returns the nearest hedgehog field behind the given position and the distance to that field.
        If there is no hedgehog field behind the given position None, None is returned.
    '''

    field_index = position

    while field_index > 0:
        field_index -= 1
        field = state.board.getField(field_index)

        if field.type == "HEDGEHOG":
            return field, position - field_index
    
    return None, None

def get_possible_moves(state, use_opponent=False):
    if use_opponent:
        player = state.opponent
        opponent = state.player
    else:
        player = state.player
        opponent = state.opponent
    
    pmvs = []

    # eat salad move
    if state.board.getField(player.position).type == "SALAD":
        move = Move()
        move.type = "eatsalad"
        pmvs.append(move)
        return pmvs
    
    # fallback move
    field, index = get_hedgehog_field(state, player.position)

    if field != None and index != opponent.index:
        move = Move()
        move.type = "fallback"
        pmvs.append(move)
    
    # exchange carrots move
    if state.board.getField(player.position).type == "CARROT":
        move = Move()
        move.type = "exchangecarrots"
        move.parameters["amount"] = "10"
        pmvs.append(move)

        if player.carrots >= 10:
            move = Move()
            move.type = "exchangecarrots"
            move.parameters["amount"] = "-10"
            pmvs.append(move)
    
    # advance move
    for i in range(player.position + 1, 65):
        
    
    return pmvs

def get_random_move(state):
    pass
   
def get_best_move(state):
    pass # TODO: implement a cool algorithm like minimax

def compute_move(state):
    #t_start = time()
    move = get_random_move(state)
    
    print(move)

    return move