from random import choice
from player import Player

from move import Move
from time import time
from copy import deepcopy

TIME_LIMIT = 1.8
LAST_SALAD_FIELD = 57
LAST_FIELD = 64
FIRST_FIELD = 1

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

def get_moves_recursive(state, player, opponent, index, incomplete_move):
    #needed_carrots = get_needed_carrots(index - player.position)
    moves = []
    field = state.board.get_field(index)

    if field.type == "HEDGEHOG":
        return []
    
    if opponent.position == index and field.type != "GOAL":
        return []
    
    if field.type == "GOAL":
        if player.carrots > 10 or player.salads > 0:
            return []
    
    if field.type == "MARKET":
        if player.carrots < 10:
            return []
        for card in ("SWAP_CARROTS", "EAT_SALAD", "HURRY_AHEAD", "FALL_BACK"):
            move = deepcopy(incomplete_move)
            move.append_card(card)
            moves.append(move)
        return moves
    
    if field.type == "HARE":
        if len(player.cards) < 1:
            return []
        if "SWAP_CARROTS" in player.cards:
            if player.position < LAST_SALAD_FIELD and opponent.position < LAST_SALAD_FIELD and state.turn - state.last_swap_carrots_turn > 2:
                move = deepcopy(incomplete_move)
                move.append_card("SWAP_CARROTS")
                moves.append(move)
        
        if "EAT_SALAD" in player.cards:
            if player.salads > 0:
                move = deepcopy(incomplete_move)
                move.append_card("EAT_SALAD")
                moves.append(move)
        
        if "HURRY_AHEAD" in player.cards:
            if index < opponent.position:
                move = deepcopy(incomplete_move)
                move.append_card("HURRY_AHEAD")
                new_player = Player(player.team, index, player.salads, player.carrots)
                new_player.cards += player.cards
                new_player.cards.remove("HURRY_AHEAD")
                moves += get_moves_recursive(state, new_player, opponent, opponent.position + 1, move)
        
        if "FALL_BACK" in player.cards:
            if index > opponent.position:
                move = deepcopy(incomplete_move)
                move.append_card("FALL_BACK")
                new_player = Player(player.team, index, player.salads, player.carrots)
                new_player.cards += player.cards
                new_player.cards.remove("FALL_BACK")
                moves += get_moves_recursive(state, new_player, opponent, opponent.position - 1, move)


        return moves
    
    moves.append(incomplete_move)
    return moves



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
    if state.board.get_field(player.position).type == "CARROTS":
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

        if opponent.position == i and field.type != "GOAL":
            continue

        if field.type == "GOAL":
            if player.carrots - needed_carrots > 10 or player.salads > 0:
                continue

        if field.type == "HARE": #TODO: check hare field conditions
            if len(player.cards) < 1:
                continue
            if "SWAP_CARROTS" in player.cards:
                if player.position < LAST_SALAD_FIELD and opponent.position < LAST_SALAD_FIELD and state.turn - state.last_swap_carrots_turn > 2 and needed_carrots <= player.carrots:
                    move = Move()
                    move.advance(i - player.position)
                    move.append_card("SWAP_CARROTS")
                    pmvs.append(move)
            
            if "EAT_SALAD" in player.cards:
                if player.salads > 0 and needed_carrots <= player.carrots:
                    move = Move()
                    move.advance(i - player.position)
                    move.append_card("EAT_SALAD")
                    pmvs.append(move)
            
            if "HURRY_AHEAD" in player.cards:
                if i < opponent.position and needed_carrots <= player.carrots and opponent.position != LAST_FIELD:
                    move = Move()
                    move.advance(i - player.position)
                    move.append_card("HURRY_AHEAD")
                    new_player = Player(player.team, i, player.salads, player.carrots - needed_carrots)
                    new_player.cards += player.cards
                    new_player.cards.remove("HURRY_AHEAD")
                    pmvs += get_moves_recursive(state, new_player, opponent, opponent.position + 1, move)
            
            if "FALL_BACK" in player.cards:
                if i > opponent.position and needed_carrots <= player.carrots and opponent.position != FIRST_FIELD:
                    move = Move()
                    move.advance(i - player.position)
                    move.append_card("FALL_BACK")
                    new_player = Player(player.team, i, player.salads, player.carrots - needed_carrots)
                    new_player.cards += player.cards
                    new_player.cards.remove("FALL_BACK")
                    pmvs += get_moves_recursive(state, new_player, opponent, opponent.position - 1, move)

            continue

        if field.type == "MARKET":
            if player.carrots < 10 + needed_carrots:
                continue
            for card in ("SWAP_CARROTS", "EAT_SALAD", "HURRY_AHEAD", "FALL_BACK"):
                move = Move()
                move.advance(i - player.position)
                move.append_card(card)
                pmvs.append(move)
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