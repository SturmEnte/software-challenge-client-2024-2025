from random import choice

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
        field = state.board.get_field(field_index)

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
    if state.board.get_field(player.position).type == "SALAD" and state.last_move.type != "eatsalad":
        move = Move()
        move.eat_salad()
        pmvs.append(move)
        return pmvs
    
    # fallback move
    field, index = get_hedgehog_field(state, player.position)

    if field != None and field.index != opponent.position:
        move = Move()
        move.fallback()
        pmvs.append(move)
    
    # exchange carrots move
    if state.board.get_field(player.position).type == "CARROT":
        move = Move()
        move.exchange_carrots(10)
        pmvs.append(move)

        if player.carrots >= 10:
            move = Move()
            move.exchange_carrots(-10)
            pmvs.append(move)
    
    # advance move
    for i in range(player.position + 1, 65):
        needed_carrots = get_needed_carrots(i - player.position)
        field = state.board.get_field(i)

        if field.type == "HEDGEHOG":
            continue

        if opponent.position == i:
            continue

        if field.type == "GOAL":
            if player.carrots > 10 or player.salads > 0:
                continue

        if field.type == "HARE": #TODO: check hare field conditions
            continue

        if field.type == "MARKET": #TODO: check market field conditions
            continue

        if (needed_carrots <= player.carrots):
            move = Move()
            move.advance(i - player.position)
            pmvs.append(move)
    
    return pmvs

def get_random_move(state):
    pmvs = get_possible_moves(state)
    return choice(pmvs)
   
def get_best_move(state):
    pass # TODO: implement a cool algorithm like minimax

def compute_move(state):
    #t_start = time()
    move = get_random_move(state)
    
    print(move)

    return move