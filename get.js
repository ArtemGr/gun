function GET(lex, graph){
  var soul = lex['#'];
  var key = lex['.'];
  var node = graph[soul];
  var tmp;
  if(!node){ return }
  if(key){
    tmp = node[key];
    if(!tmp){ return }
    (node = {_: node._})[key] = tmp;
    tmp = node._['>'];
    (node._['>'] = {})[key] = tmp[key];
  }
  var ack = {};
  ack[soul] = node;
  return ack;
}

try{module.exports = GET}catch(e){}
