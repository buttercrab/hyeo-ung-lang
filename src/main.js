function check_parse(){
    var input1 = document.getElementById("unum1").value
    var input2 = document.getElementById("unum2").value
    let un_opt_code = parse(input1);
    var un_opt_state = new UnOptState();
    let stdout = new Output();
    let stderr = new Output();
    let ipt = new Output();
    ipt.adding(input2.toString());

    for(let c of un_opt_code){
        un_opt_state = execute(ipt, stdout, stderr, un_opt_state, c);
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
