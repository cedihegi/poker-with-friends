class Authen_local{
    constructor(){
        this._auth = false;
        this._name = null;
    }

    set_name(nm){
        this._name = nm;
        this._auth = true;
    }

    get_auth(){
        return this._auth;
    }
}

