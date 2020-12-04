function Dup(){
    var dup = {s:{}}, opt = {max: 1000, age: 1000 * 9};
    dup.check = function(id){
        return dup.s[id]? dup.track(id) : false;
    }
    dup.track = function(id){
        dup.s[id] = (+new Date());
        if(!dup.to){
            dup.to = setTimeout(function(){
                Object.keys(dup.s).forEach(function(id){
                    var time = dup.s[id];
                    if(opt.age > ((+new Date()) - time)){ return }
                    delete dup.s[id];
                });
                dup.to = null;
            }, opt.age);
        }
        return id;
    }
    return dup;
}

Dup.random = function(){ return Math.random().toString(36).slice(-3) }
try{module.exports = Dup}catch(e){}
