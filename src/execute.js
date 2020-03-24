function push_stack_wrap(out, err, state, idx, num) {
    if(idx === 1) {
        if(num > 0){
            out.adding(String.fromCharCode(Math.floor(num)));
        }
        else {
            out.adding((-num).toString());
        }
    }
    else if(idx === 2) {
        if(num > 0){
            err.adding(String.fromCharCode(Math.floor(num)));
        }
        else {
            err.adding((-num).toString());
        }
    }
    else {
        state.push_stack(idx, num);
    }
}

function pop_stack_wrap(ipt, out, err, state, idx) {
    if(idx === 0) {
        if(state.get_stack(0).length === 0){
            let s = ipt.toString();
            for(let c of s){
                state.push_stack(0, parseInt(c));
            }
        }
        return state.pop_stack(0);
    }
    else if(idx ===1) {
        alert("exit 1");
    }
    else if(idx === 2){
        alert("exit 2");
    }
    else {
        return state.pop_stack(idx);
    }
}

function execute_one(ipt, out, err, state, cur_loc){
    let code = clone(state.get_code(cur_loc));
    let cur_stack = state.current_stack();

    if(code.type === 0){
        push_stack_wrap(out, err, state, cur_stack, (code.hangul_count)*(code.dot_count));
    }
    else if(code.type === 1){
        let n = 0;
        for(let i = 0; i < code.hangul_count ; i++){
            n += pop_stack_wrap(ipt, out, err, state, cur_stack); 
        }
        push_stack_wrap(out, err, state, code.dot_count, n);
    }
    else if(code.type === 2){
        let n = 1;
        for(let i = 0; i < code.hangul_count ; i++){
            n *= pop_stack_wrap(ipt, out, err, state, cur_stack); 
        }
        push_stack_wrap(out, err, state, code.dot_count, n);
    }
    else if(code.type === 3){
        let n = 0;
        let v = new Array();
        for(let i = 0; i < code.hangul_count ; i++){
            v.push(pop_stack_wrap(ipt, out, err, state, cur_stack));
        }
        for(let x of v){
            n -= x;
            push_stack_wrap(out, err, state, cur_stack, x);
        }
        push_stack_wrap(out, err, state, code.dot_count, n);
    }
    else if(code.type === 4){
        let n = 1;
        let v = new Array();
        for(let i = 0; i < code.hangul_count ; i++){
            v.push(pop_stack_wrap(ipt, out, err, state, cur_stack));
        }
        for(let x of v){
            n *= (1/x);
            push_stack_wrap(out, err, state, cur_stack, 1/x);
        }
        push_stack_wrap(out, err, state, code.dot_count, n);
    }
    else {
        let n = pop_stack_wrap(ipt, out, err, state, cur_stack);
        for(let i = 0; i < code.hangul_count ; i++){
            push_stack_wrap(out, err, state, code.dot_count, n);
        }
        push_stack_wrap(out, err, state, cur_stack, n);
        state.set_current_stack(code.dot_count);
    }

    cur_stack = state.current_stack();
    let n = pop_stack_wrap(ipt, out, err, state, cur_stack);
    let area_type = calc(code.area, code.area_count(), n);
    push_stack_wrap(out, err, state, cur_stack, n);

    if(area_type != 0){
        if(area_type != 13){
            let id = ((code.area_count()) << 4) + area_type;
            let value = state.get_point(id);
            if(value === undefined){
                state.set_point(id, cur_loc);
            }
            else{
                if(cur_loc !== value){
                    state.latest = cur_loc;
                    return [state, value];
                }
            }
        }
        else{
            let loc = state.latest;
            if(loc !== -1){
                return [state, loc];
            }
        }
    }

    return [state, cur_loc + 1];
}

function execute(ipt, out, err, state, code){
    let cur_loc = state.push_code(clone(code));
    let length = cur_loc + 1;

    while(cur_loc < length){
        let [new_state, new_loc] = execute_one(ipt, out, err, state, cur_loc);
        state = new_state;
        cur_loc = new_loc;
    }

    return state;
}

