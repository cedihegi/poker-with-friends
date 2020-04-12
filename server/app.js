const http = require('http');
const express = require('express');
const socketio = require('socket.io');
const Authen = require('./js/authen')

const app = express();


const client_path = `${__dirname}/../client`;
console.log(`Serving static from ${client_path}`);
app.use(express.static(client_path));

const server = http.createServer(app);

const io = socketio(server);



server.on('error', (err) => {
    console.error('Server error: ', err);
});

server.listen(8080, () => {
    console.log(`Server running on 8080`);
});



// Handling io-events:

//Connection with a client:
io.on('connection', (sock) => {
    console.log("someone connected");
    sock.emit('message', 'Hi you are connected');

    var a = new Authen();

    sock.on('message', (text) => {
        if(a.get_auth() == true){
            var name = a._name;
            io.emit('message', (name + ": " + text));
        } else {
            sock.emit('message', "can't send messages without authenticating");
        }
    })

    sock.on('auth', (text) => {
        console.log("user authenticated himself as", text)
        a.set_name(text);
    })

    
});