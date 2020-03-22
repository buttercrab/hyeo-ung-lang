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
    get area_count(){
        return this.hangul_count*this.dot_count;
    }
}

class Area {
    constructor(type, left, right, Nill) {
        this.type = type;
        this.left = left;
        this.right = right;
        this.Nil = Nill;
    }
}

class UnOptState {
    constructor(){
        this.stack = new Map();
        this.code = new Array();
        this.point = new Map();
        this.cur = 3;
    }
}