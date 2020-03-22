function is_hangul_syllable(c) {
    return '\u{AC00}' <= c && c <= '\u{D7A3}'
}

function parse(input){
    var res = new Array();
    var hangul_count = 0;
    var dot_count = 0;
    var type = 10;
    var loc = [1, 0];
    var state = 0;
    var area = new Area(10,Object,Object,false);
    var leaf = area;
    var qu_area = new Area(10,Object,Object,false);
    var qu_leaf = qu_area;

    var line_count = 0;
    var last_line_started = 0;
    var raw_command = new String("");

    var max_pos = [0, 0, 0];
    for(var i = 0; i < input.length; i++) {
        var t = "엉앙앗읏읍윽".indexOf(input[i]);
        if(t >= 0){
            if(t == 0) max_pos[0] = i;
            else if(t <= 6) max_pos[1] = i;
            else  max_pos[2] = i;
        }
    }

    for(var i = 0; i < input.length; i++) {
        var c = String.fromCodePoint(input.codePointAt(i));
        if(c == ' ') continue;
        if(c == '\n'){
            line_count += 1;
            last_line_started = i + 1;      
            continue;      
        }
        if(state == 0 || state == 2){
            var t1 = "형항핫흣흡흑혀하흐".indexOf(c);
            var t2 = "♥❤💕💖💗💘💙💚💛💜💝♡".indexOf(c);
            if(t1 >= 0){
                if(t1 >= 6 && max_pos[t1 - 6] <= i) {
                    continue;
                }
                if(type != 10) {
                    var temp_area = area;
                    if(qu_leaf.Nil){
                        qu_leaf.right = area;
                        temp_area = qu_area;
                    }
                    res.push(new UnOptCode(type,hangul_count,dot_count,loc,temp_area,raw_command));
                    area = new Area(10,Object,Object,false);
                    leaf = area;
                    qu_area = new Area(10,Object,Object,false);
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
                if(c == '.') { 
                    dot_count += 1 
                } 
                else { 
                    dot_count += 3 
                };
                raw_command.concat(c);
            }
            else if(c == '?'){
                if(qu_leaf.Nil){
                    qu_leaf.right = new Area(0,area,new Area(10,Object,Object,false),true);
                    qu_leaf = qu_leaf.right;
                }
                else{
                    qu_area = new Area(0,area,new Area(10,Object,Object,false),true);
                    qu_leaf = qu_area;
                }
                area = new Area(10,Object,Object,false);
                leaf = area;
                raw_command.concat(c);
                state = 2;
            }
            else if(c == '!'){
                if(leaf.Nil){
                    if(leaf.type <= 1){
                        if(leaf.right.Nil){
                            leaf.right = new Area(1,new Area(leaf.right.type,Object,Object,true),new Area(10,Object,Object,false),true);
                            leaf = leaf.right;
                        }
                        else{
                            area = new Area(1,Area(leaf.type,Object,Object,true),new Area(10,Object,Object,false),true);
                            leaf = area;
                        }
                    }
                    leaf = leaf.right;
                }
                else{
                    area = new Area(1,new Area(10,Object,Object,false),new Area(10,Object,Object,false),true);
                    leaf = area;
                }
            }
            else if(t2 >= 0){
                if((t2%2)&(t2>=3)&(t2<=19)) continue;
                if(t2>3){
                    t2/=2;
                    t2++;
                }
                t2+=2;
                if(leaf.Nil&&leaf.type<=1){
                    if(!leaf.right.Nil){
                        leaf.right = new Area(t2,new Area(10,Object,Object,false),new Area(10,Object,Object,false),true);
                    }
                }
                else{
                    area = new Area(t2,new Area(10,Object,Object,false),new Area(10,Object,Object,false),true);
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
            if(type == 6){
                if("엉".indexOf(c)>=0) {
                    type_ = 0;
                    dot_count = 0;
                    state = 0;
                } 
                else {
                    state = 1;
                }
            }
            if(type == 7){
                if("앙앗".indexOf(c)>=0) {
                    type_ = t/3 + 1;
                    dot_count = 0;
                    state = 0;
                } 
                else {
                    state = 1;
                }
            }
            if(type == 8){
                if("읏읍윽".indexOf(c)>=0) {
                    type_ = t/3 + 3;
                    dot_count = 0;
                    state = 0;
                } 
                else {
                    state = 1;
                }
            }
        }
    }
    if(type != 10) {
        var temp_area = area;
        if(qu_leaf.Nil){
            qu_leaf.right = area;
            temp_area = qu_area;
        }
        res.push(new UnOptCode(type,hangul_count,dot_count,loc,temp_area,raw_command));
    }
    for(var i=0;i<res.length;i++){
        console.log(res[i].area.type);
    }
}