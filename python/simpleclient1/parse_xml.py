from board import Board
from field import Field
from player import Player
from xml.etree import ElementTree

def parse_board(board_tag):
    board = Board()

    for index, field_tag in enumerate(board_tag):

        field_type = field_tag.text
        field_index = index

        field = Field(field_type, field_index)
        
        board.append_field(field)

    return board

def parse_players(received_players):
    players = []
    for received_player in received_players:

        player = Player(
            received_player.attrib['team'],
            int(received_player.attrib['position']),
            int(received_player.attrib['salads']),
            int(received_player.attrib['carrots'])
        )

        players.append(player)
    
    return players

def parse_memento_start(state):
    start_team = state.attrib['startTeam']
    board_tag = state.find('board')

    # parse board
    board = parse_board(board_tag)
    
    # parse players
    players = parse_players(state.findall('hare'))

    return start_team, board, players

def parse_memento(state):
    # parse players
    players = parse_players(state.findall('hare'))

    return players

def parse_result(data, state):
    score_one = data.find('scores').findall('entry')[0]
    score_two = data.find('scores').findall('entry')[1]

    winner = data.find('winner')

    if winner.attrib['team'] == "ONE":
        team = "ONE"
        playername = score_one.find('player').attrib['name']
    else:
        team = "TWO"
        playername = score_two.find('player').attrib['name']
    
    if state.team == "ONE":
        our_score = score_one
        opponent_score = score_two
    else:
        our_score = score_two
        opponent_score = score_one

    

    regular = winner.attrib['regular']
    reason = winner.attrib['reason']

    our_stats = our_score.find('score').findall('part')
    opponent_stats = opponent_score.find('score').findall('part')
    
    csv = f'{state.start_team},{state.team},{state.opponent.team},{team},{regular},{our_stats[0].text},{our_stats[1].text},{our_stats[2].text},{opponent_stats[0].text},{opponent_stats[1].text},{opponent_stats[2].text},{reason}'
    
    result = f'''--------------Result---------------
WINNER: {playername} (Team {team})

Regular: {regular}
Reason: {reason}

----------Player----------
Siegespunkte: {our_stats[0].text}
Position:     {our_stats[1].text}
Karotten:     {our_stats[2].text}

---------Opponent---------
Siegespunkte: {opponent_stats[0].text}
Position:     {opponent_stats[1].text}
Karotten:     {opponent_stats[2].text}

-----------------------------------'''

    return result, csv

def parse_error(data):
    message = data.attrib['message']
    return f'---------------Error---------------\nError message from server:\n{message}\n-----------------------------------'