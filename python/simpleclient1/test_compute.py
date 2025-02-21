from parse_xml import parse_memento_start # parse_memento_start is imported, because it also returns the start team
from time import time
from state import State
from xml.etree.ElementTree import fromstring
from compute import compute_move, get_possible_moves

#TODO: make it work with entire replay and specify turn per variable/input
#      this has to be done to correctly get the last move for eat_salad calculation and for last use of exchange carrots card

with open("../test/memento1.xml", "r") as msg:
    msg = msg.read()
    print("\nNEW MESSAGE:\n" + msg + "\n" + "-"*35)

    msg  = fromstring(msg)

    data = msg.find('data')
    msgType = data.attrib['class']
    
    t1 = time()
    xmlState = data.find('state')
    turn = int(xmlState.attrib['turn'])

    start_team, board, players = parse_memento_start(xmlState)
    state = State("TWO", turn, start_team, board, players)

    move = compute_move(state)

    t2 = time()
    print(f"Zeit: {t2-t1}   Zug: {turn}")
    state.print_state()
    print(move)

    print("-"*35)
    print("Possible Moves:")
    pmvs = get_possible_moves(state)
    for mv in pmvs:
        print(mv)