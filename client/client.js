function writeEvent(text) {
    const parent = document.querySelector('#events');

    const el = document.createElement('li');
    el.innerHTML = text;

    parent.appendChild(el);
}

function onFormSubmitted(e){
    console.log("submitted");
    e.preventDefault();

    const input = document.querySelector('#chat');
    const text = input.value;
    input.value = '';

    sock.emit('message', text);
}

function onAuthSubmitted(e){
    console.log("login");
    e.preventDefault();

    const input = document.querySelector('#namefield');
    const text = input.value;
    input.value = '';

    sock.emit('auth', text)
    a.set_name(text);
    localStorage.setItem("authen", a);
}





writeEvent('Welcome to RPS');

//variables to keep state on reload
var a = localStorage.getItem("authen");
if(a==null){
    a = new Authen_local();
}

const sock = io();
sock.on('message', writeEvent);


document
    .querySelector('#chat-form')
    .addEventListener('submit', onFormSubmitted);

document
    .querySelector('#auth-form')
    .addEventListener('submit', onAuthSubmitted);