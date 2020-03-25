function is_hangul_syllable(c) {
    return '\u{AC00}' <= c && c <= '\u{D7A3}'
}

function parse(input){
    let res = new Array();
    let hangul_count = 0;
    let dot_count = 0;
    let type = 10;
    let loc = [1, 0];
    let state = 0;
    let area = null;
    let leaf = area;
    let qu_area = null;
    let qu_leaf = qu_area;

    let line_count = 0;
    let last_line_started = 0;
    let raw_command = new String("");

    let max_pos = [0, 0, 0];
    for(let i = 0; i < input.length; i++) {
        let c = String.fromCodePoint(input.codePointAt(i));
        let t = "엉앙앗읏읍윽".indexOf(c);
        if(t >= 0){
            if(t === 0) max_pos[0] = i;
            else if(t <= 2) max_pos[1] = i;
            else  max_pos[2] = i;
        }
    }

    for(let i = 0; i < input.length; i++) {
        let c = String.fromCodePoint(input.codePointAt(i));
        if(c === ' ') continue;
        if(c === '\n'){
            line_count += 1;
            last_line_started = i + 1;      
            continue;      
        }
        if(state === 0 || state === 2){
            let t1 = "형항핫흣흡흑혀하흐".indexOf(c);
            let t2 = "♥❤💕💖💗💘💙💚💛💜💝♡".indexOf(c);
            if(t1 >= 0){
                if(t1 >= 6 && max_pos[t1 - 6] <= i) {
                    continue;
                }
                if(type !== 10) {
                    let temp_area;
                    if(qu_leaf !== null){
                        qu_leaf.right = area;
                        temp_area = qu_area;
                    }
                    else{
                        temp_area = area;
                    }
                    res.push(new UnOptCode(type,hangul_count,dot_count,loc,temp_area,raw_command));
                    area = null;
                    leaf = area;
                    qu_area = null;
                    qu_leaf = qu_area;
                }
                type = t1;
                hangul_count = 1;
                dot_count = 0;
                loc = [line_count + 1, i - last_line_started];
                raw_command = new String(c);
                if(t1 < 6) {
                    state = 0;
                } 
                else {
                    state = 1;
                }
            }
            else if(".…⋯⋮".indexOf(c) >= 0){
                if(state === 0){
                    if(c === '.') { 
                        dot_count += 1 
                    } 
                    else { 
                        dot_count += 3 
                    };
                    raw_command.concat(c);
                }                
            }
            else if(c === '?'){
                if(qu_leaf !== null){
                    qu_leaf.right = new Area(0,area,null);
                    qu_leaf = qu_leaf.right;
                }
                else{
                    qu_area = new Area(0,area,null);
                    qu_leaf = qu_area;
                }
                area = null;
                leaf = area;
                raw_command.concat(c);
                state = 2;
            }
            else if(c === '!'){
                if(leaf !== null){
                    if(leaf.type <= 1){
                        if(leaf.right !== null){
                            leaf.right = new Area(1,new Area(leaf.right.type,null,null),null);
                        }
                        else{
                            area = new Area(1,null,null);
                        }
                        leaf = leaf.right;
                    }
                    else{
                        area = new Area(1,new Area(leaf.type,null,null),null);
                        leaf = area;
                    }
                }
                else{
                    area = new Area(1,null,null);
                    leaf = area;
                }
                raw_command.concat(c);
                state = 2;
            }
            else if(t2 >= 0){
                if((t2%2)&(t2>=3)&(t2<=19)) continue;
                if(t2>3){
                    t2/=2;
                    t2++;
                }
                t2+=2;
                if(leaf !== null&& leaf.type<=1){
                    if(leaf.right === null){
                        leaf.right = new Area(t2,null,null);
                    }
                }
                else{
                    area = new Area(t2,null,null);
                    leaf = area;
                }
                raw_command.concat(c);
                state = 2;
            }
            else continue;
        }
        else {
            if(is_hangul_syllable(c)){
                hangul_count += 1;
                raw_command.concat(c);
            }
            if(type === 6){
                let t = "엉".indexOf(c);
                if(t >= 0) {
                    type = 0;
                    dot_count = 0;
                    state = 0;
                } 
                else {
                    state = 1;
                }
            }
            if(type === 7){
                let t = "앙앗".indexOf(c);
                if(t >= 0) {
                    type = t + 1;
                    dot_count = 0;
                    state = 0;
                } 
                else {
                    state = 1;
                }
            }
            if(type === 8){
                let t = "읏읍윽".indexOf(c);
                if(t >= 0) {
                    type = t + 3;
                    dot_count = 0;
                    state = 0;
                } 
                else {
                    state = 1;
                }
            }
        }
    }
    if(type !== 10) {
        let temp_area;
        if(qu_leaf !== null){
            qu_leaf.right = area;
            temp_area = qu_area;
        }
        else temp_area = area;
        res.push(new UnOptCode(type,hangul_count,dot_count,loc,temp_area,raw_command));
    }
    return res;
}

