# TODO: manage coal when turning
# TODO: prevent ship from reaching speeds below 1

from move import Move
from time import time

TIME_LIMIT = 1.8

def getPossibleMoves(state):
    
    pmvs = getPossibleMovesRecursive(state.player.getPosition(), state.player.direction, state.player.coal, state.player.freeTurns, min_speed, max_speed)

    return pmvs

def getRandomMove(state):
    return getAStarMove(state, positionDictToTuple(state.player.position), state.board.getRandomCoords())
   
def getBestMove(state):
    return getAStarMove(state, positionDictToTuple(state.player.position), state.board.farthestField)

def computeMove(state):
    t_start = time()
    move, move_possible, coalNeeded = getBestMove(state)

    bestMove = move

    if not move_possible:
        while time() - t_start < TIME_LIMIT:
            move, move_possible, coalNeeded = getNextMove(state)
            if move_possible and coalNeeded < minCoalNeeded:
                minCoalNeeded = coalNeeded
                bestMove = move


    # print(f"movementPoints: {movementPoints}, freeTurns: {freeTurns}, coalNeeded: {coalNeeded}")
    print(bestMove)

    return bestMove