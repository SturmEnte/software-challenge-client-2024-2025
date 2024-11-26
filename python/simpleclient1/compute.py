# TODO: manage coal when turning
# TODO: prevent ship from reaching speeds below 1

from move import Move
from time import time

TIME_LIMIT = 1.8

def getPossibleMoves(state):
    
    pmvs = getPossibleMovesRecursive(state.player.getPosition(), state.player.direction, state.player.coal, state.player.freeTurns, min_speed, max_speed)

    return pmvs

def getRandomMove(state):
    pass
   
def getBestMove(state):
    pass # TODO: implement a cool algorithm like minimax

def computeMove(state):
    t_start = time()
    move = getRandomMove(state)

    


    # print(f"movementPoints: {movementPoints}, freeTurns: {freeTurns}, coalNeeded: {coalNeeded}")
    print(move)

    return move