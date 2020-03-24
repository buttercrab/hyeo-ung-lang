class OptCode {
    constructor(type , hangul_count, dot_count, area_count, area) {
        this.type = type;
        this.hangul_count = hangul_count;
        this.dot_count = dot_count;
        this.area_count = area_count;
        this.area = area;
    }
}

class UnOptCode {
    constructor(type, hangul_count, dot_count, loc, area, code) {
        this.type = type;
        this.hangul_count = hangul_count;
        this.dot_count = dot_count;
        this.loc = loc;
        this.area = area;
        this.code = code;
    }
    area_count(){
        return this.hangul_count*this.dot_count;
    }
}

class Area {
    constructor(type, left, right) {
        this.type = type;
        this.left = left;
        this.right = right;
    }
}

function calc(area, area_value, ipt, out, err, state, cur_stack){
    while(1){
        if(area !== null){
            if(area.type === 0){
                let pop = pop_stack_wrap(ipt, out, err, state, cur_stack);
                if(pop < area_value){
                    area = area.left;
                }
                else{
                    area = area.right;
                }
            }
            else if(area.type === 1){
                let pop = pop_stack_wrap(ipt, out, err, state, cur_stack);
                if(pop === area_value){
                    area = area.left;
                }
                else{
                    area = area.right;
                }
            }
            else{
                return area.type;
            }
        }
        else{
            return 0;
        }
    }
}

class UnOptState {
    constructor(){
        this.stack = new Map();
        this.code = new Array();
        this.point = new Map();
        this.cur = 3;
        this.latest = -1;
    }

    get_stack(){
        return this.stack;
    }

    get_all_stack_index(){
        let v = new Array();
        for(let [key, value] of this.stack){
            v.push(clone(key));
        }
        return v;
    }

    stack_size(){
        return this.stack.size;
    }

    current_stack(){
        return this.cur;
    }

    set_current_stack(cur){
        this.cur = cur;
    }

    get_code(loc){
        return this.code[loc];
    }

    push_code(code){
        this.code.push(code);
        return this.code.length - 1;
    }

    get_all_code(){
        return clone(this.code);
    }

    set_point(id, loc){
        this.point.set(id, loc);
    }

    get_point(id){
        return this.point.get(id);
    }

    get_all_point(){
        let v = new Array();
        for(let [a, b] of this.point){
            v.push([a, b]);
        }
        return v;
    }

    push_stack(idx, num){
        if(this.stack.get(idx) == undefined){
            this.stack.set(idx,new Array());
        }
        if(this.stack.get(idx).length !== 0 || num !== NaN){
            this.stack.get(idx).push(num);
        }
    }

    pop_stack(idx){
        if(this.stack.get(idx) === undefined){
            this.stack.set(idx,new Array());
        }
        let t = this.stack.get(idx).pop();
        if(t === undefined) return NaN;
        else return t;
    }
}

class Output {
    constructor(){
        this.out = new String("");
    }

    adding(str){
        this.out = this.out.concat(str);
    }
}