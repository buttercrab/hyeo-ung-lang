function check_parse(){
    var input = document.getElementById("unum1").value
    let un_opt_code = parse(input);
    var un_opt_state = new UnOptState();
    let stdout = new Output();
    let stderr = new Output();

    for(let c of un_opt_code){
        un_opt_state = execute("", stdout, stderr, un_opt_state, c);
    }
    
    console.log(stdout.out);
    console.log(stderr.out);
}

function clone(obj) {
    if (obj === null || typeof(obj) !== 'object')
    return obj;
  
    var copy = new obj.constructor();
  
    for (var attr in obj) {
      if (obj.hasOwnProperty(attr)) {
        copy[attr] = obj[attr];
      }
    }
  
    return copy;
}
