# pip install pyyaml
import yaml
import subprocess
import os
import signal
from time import sleep

RESULTS_FILE = "results.csv"
ANIMATION = "|/-\\"

def parse_value(line, name):
    line += ",])" # to prevent errors from getting distance_to_end
    index = line.index(name)
    length = len(name)
    distance_to_end = min(line[index+length+1:].index(","), line[index+length+1:].index("]"), line[index+length+1:].index(")"))
    value = line[index+length+1 : index+length+1 + distance_to_end]
    return value

def start_clients(swapped=False):
    # wait for server to start / clients to disconnect from previous game
    sleep(4)

    if not swapped:
        # start first client
        subprocess.Popen(config["client_1"]["start_command"], shell=True, stdout=subprocess.PIPE)

        sleep(5)

        # start second client
        subprocess.Popen(config["client_2"]["start_command"], shell=True, stdout=subprocess.PIPE)

    else:
        # start second client
        subprocess.Popen(config["client_2"]["start_command"], shell=True, stdout=subprocess.PIPE)

        sleep(5)

        # start first client
        subprocess.Popen(config["client_1"]["start_command"], shell=True, stdout=subprocess.PIPE)

def print_result(stats):
    print("----------------------------------")
    print(f"\nStats for {config['client_1']['display_name']} for the {games_played} games played:\n\n")
    print("Average values:\n")
    
    for stat in ("rounds", "position", "carrots", "salads"):
        sum = 0
        for value in stats[stat]:
            sum += value
        average = sum / len(stats[stat])
        print(f"    {stat.capitalize()+':': <10}{average}")
    
    print(f"\nIrregular Results: {stats['irregular_results']}")
    print(f"Games Won: {stats['games_won']}/{games_played}\n")
    print("----------------------------------")

with open("config.yaml") as file:
    config = yaml.safe_load(file)

if not os.path.exists(RESULTS_FILE):
    with open(RESULTS_FILE, "w") as csv:
        csv.write("Rounds,Turns,Winner,Regular,P1 Points,P1 Position,P1 Carrots,P1 Salads,P2 Points,P2 Position,P2 Carrots,P2 Salads,Winner Message\n")

played_games_stats = {"rounds": [], "position": [], "carrots": [], "salads": [], "irregular_results": 0, "games_won": 0}

player1 = {}
player2 = {}

turn = None
rounds = None
regular = None
winner_message = None
winner = None

log = subprocess.Popen("java -jar software-challenge-server/server.jar --port 13051", shell=True, stdout=subprocess.PIPE, stderr=subprocess.PIPE)

games_played = 0
animation_index = 0

start_clients()
game_running = True

for stdout_line in iter(log.stdout.readline, ""):
    line = stdout_line.decode()

    print(f"Game {games_played + 1} running", ANIMATION[animation_index], end="\r")
    animation_index = (animation_index + 1) % len(ANIMATION)

    if "Starting Game" in line:
        game_running = True
    if "Game(OVER" in line and "regular" in line and game_running:
        #, players=[ONE(), TWO()], gameState=GameState(board=Board[\xe2\x96\xb6, \xf0\x9f\xa5\x95, \xf0\x9f\xa5\x95, \xf0\x9f\x90\x87, \xf0\x9f\x90\x87, \xf0\x9f\xa5\x95, \xf0\x9f\x90\x87, P1, P2, \xf0\x9f\xa7\xba, \xf0\x9f\xa5\x97, \xf0\x9f\xa6\x94, \xf0\x9f\xa5\x95, \xf0\x9f\x90\x87, \xf0\x9f\xa5\x95, \xf0\x9f\xa6\x94, P2, P1, \xf0\x9f\xa7\xba, \xf0\x9f\xa6\x94, \xf0\x9f\xa5\x95, P2, \xf0\x9f\xa5\x97, \xf0\x9f\xa5\x95, \xf0\x9f\xa6\x94, P2, \xf0\x9f\xa7\xba, \xf0\x9f\xa5\x95, \xf0\x9f\xa5\x95, \xf0\x9f\x90\x87, \xf0\x9f\xa6\x94, \xf0\x9f\xa5\x95, \xf0\x9f\xa5\x95, P1, \xf0\x9f\x90\x87, \xf0\x9f\x90\x87, P2, \xf0\x9f\xa6\x94, \xf0\x9f\x90\x87, \xf0\x9f\xa5\x95, \xf0\x9f\xa5\x95, P2, \xf0\x9f\xa5\x97, \xf0\x9f\xa6\x94, \xf0\x9f\x90\x87, \xf0\x9f\xa7\xba, P2, \xf0\x9f\xa5\x95, P1, \xf0\x9f\xa5\x95, \xf0\x9f\xa6\x94, \xf0\x9f\xa5\x95, P2, \xf0\x9f\x90\x87, \xf0\x9f\xa5\x95, \xf0\x9f\xa7\xba, \xf0\x9f\xa6\x94, \xf0\x9f\xa5\x97, \xf0\x9f\x90\x87, \xf0\x9f\xa5\x95, P1, \xf0\x9f\xa5\x95, \xf0\x9f\x90\x87, \xf0\x9f\xa5\x95, \xf0\x9f\x8f\x81], turn=48, players=[Hare(team=ONE, position=64, salads=0, carrots=3, lastAction=Vorw\xc3\xa4rts um 9, cards=[EAT_SALAD, EAT_SALAD]), Hare(team=TWO, position=54, salads=5, carrots=17, lastAction=Vorw\xc3\xa4rts um 3, cards=[HURRY_AHEAD])], lastMove=Vorw\xc3\xa4rts um 3)) is over (regular=true)\n

        turn = int(parse_value(line, "turn"))
        rounds = int(turn/2)

        player1["salads"] = int(parse_value(line, "salads"))

        # done with player one values, moving on to player two
        line = line[line.index("salads") + 6:]

        player2["salads"] = int(parse_value(line, "salads"))
            
    if "data=GameResult" in line and game_running:
        #(winner=ONE hat das Ziel zuerst erreicht., scores=[[Siegpunkte=2, Feldnummer=64, Karotten=3], [Siegpunkte=0, Feldnummer=54, Karotten=17]])) via TcpNetwork{socket=Socket[addr=/127.0.0.1,port=54550,localport=13051]} from Client@2249c889\n
        
        winner_message = line[line.index("winner")+7:line.index(", scores")]

        if "hat das Ziel zuerst erreicht" in winner_message or "ist weiter vorne" in winner_message or "Beide Spieler sind gleichauf" in winner_message:
            regular = True
        else:
            regular = False
        
        player1["points"] = int(parse_value(line, "Siegpunkte"))
        player1["position"] = int(parse_value(line, "Feldnummer"))
        player1["carrots"] = int(parse_value(line, "Karotten"))

        # done with player one values, moving on to player two
        line = line[line.index("Karotten") + 8:]

        player2["points"] = int(parse_value(line, "Siegpunkte"))
        player2["position"] = int(parse_value(line, "Feldnummer"))
        player2["carrots"] = int(parse_value(line, "Karotten"))

        if player1["points"] == 2:
            winner = "ONE"
        elif player2["points"] == 2:
            winner = "TWO"
        else:
            winner = "None"

        #print(winner, winner_message, player1, player2, regular, turn, rounds)

        played_games_stats["rounds"].append(rounds)
        if not regular:
            played_games_stats["irregular_results"] += 1

        if not (config["swap_after_half"] and games_played >= config["games"]/2):
            played_games_stats["position"].append(player1["position"])
            played_games_stats["carrots"].append(player1["carrots"])
            played_games_stats["salads"].append(player1["salads"])
            if winner == "ONE":
                played_games_stats["games_won"] += 1
        else:
            played_games_stats["position"].append(player2["position"])
            played_games_stats["carrots"].append(player2["carrots"])
            played_games_stats["salads"].append(player2["salads"])
            if winner == "TWO":
                played_games_stats["games_won"] += 1

        with open(RESULTS_FILE, "a") as csv:
            csv.write(f"{rounds},{turn},{winner},{regular},{player1['points']},{player1['position']},{player1['carrots']},{player1['salads']},{player2['points']},{player2['position']},{player2['carrots']},{player2['salads']},{winner_message}\n")

        games_played += 1
        print(f"Game {games_played} completed...")

        if games_played == config["games"]:

            print_result(played_games_stats)

            break

        game_running = False
        
        start_clients(config["swap_after_half"] and games_played >= config["games"]/2)

# close the server, once all games are played
os.killpg(os.getpgid(log.pid), signal.SIGTERM)