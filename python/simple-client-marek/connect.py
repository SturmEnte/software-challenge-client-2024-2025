
import socket



class ServerConnection ():

    def __init__(self, hostname = 'localhost', socketNummber = 13050):
        self.hostname = hostname
        self.socketNummber = socketNummber
        self.RId = None
        self.resCode = None
        self.client = socket.socket(socket.AF_INET, socket.SOCK_STREAM)
        self.client.connect((hostname, socketNummber))
        
        

    def joinRoom(self):
             
        if(self.RId == None and self.resCode == None):
            self.client.send("<protocol><join gameType='swc_2025_hase_und_igel'/>".encode())

            pass
        elif(self.RId != None and self.resCode == None):
            self.client.send("<protocol><joinRoom roomId={RId}>".encode())
            pass
        else:
            self.client.send("<protocol><joinPrepared reservationCode={resCode}>".encode())
            pass

    def receiveData(self):
        data = self.client.recv(1022)
        print(data.decode())
        pass

# Temporary Main
serverConnection = ServerConnection()
serverConnection.joinRoom()

while True:
    serverConnection.receiveData()