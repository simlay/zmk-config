// ─── Word list (1000 most common English words) ───
pub const WORDS: &[&str] = &[
    "the","be","to","of","and","a","in","that","have","i","it","for","not","on","with","he","as",
    "you","do","at","this","but","his","by","from","they","we","say","her","she","or","an","will",
    "my","one","all","would","there","their","what","so","up","out","if","about","who","get","which",
    "go","me","when","make","can","like","time","no","just","him","know","take","people","into",
    "year","your","good","some","could","them","see","other","than","then","now","look","only",
    "come","its","over","think","also","back","after","use","two","how","our","work","first","well",
    "way","even","new","want","because","any","these","give","day","most","us","large","must","big",
    "high","such","follow","act","why","ask","men","change","went","light","kind","off","need",
    "house","picture","try","again","animal","point","mother","world","near","build","self","earth",
    "father","head","stand","own","page","should","country","found","answer","school","grow","study",
    "still","learn","plant","cover","food","sun","four","thought","meant","run","problem","whole",
    "noise","always","begin","seem","play","everything","between","though","turn","finish","round",
    "life","nothing","tell","show","care","little","down","put","top","left","child","east","night",
    "reach","wide","read","hold","until","move","become","true","feel","right","word","fish","chief",
    "late","start","matter","buy","above","sleep","same","enough","maybe","pull","fall","hand","case",
    "around","gone","game","check","became","green","opposite","natural","market","never","reason",
    "produce","city","town","river","mountain","road","sea","lake","forest","garden","street","building",
    "office","factory","shop","hospital","library","museum","theater","restaurant","hotel","airport",
    "station","church","class","teacher","student","lesson","book","paper","pen","pencil","desk","chair",
    "table","window","door","wall","floor","ceiling","room","home","family","sister","brother","baby",
    "man","woman","boy","girl","friend","person","group","team","party","meeting","job","career",
    "money","price","cost","value","trade","business","company","bank","store","water","bread","meat",
    "fruit","vegetable","milk","egg","cake","coffee","tea","beer","wine","drink","eat","cook","kitchen",
    "plate","cup","bottle","clothes","shirt","shoes","hat","dress","coat","sock","belt","bag","watch",
    "car","bus","train","plane","bike","boat","ship","bridge","tunnel","village","countryside","state",
    "nation","ocean","island","spring","summer","autumn","winter","season","weather","rain","snow","wind",
    "moon","star","sky","cloud","tree","flower","grass","leaf","root","branch","bird","dog","cat","horse",
    "cow","sheep","pig","bear","lion","tiger","elephant","monkey","snake","rabbit","mouse","deer","wolf",
    "fox","love","hate","hope","dream","fear","joy","anger","sad","happy","mad","glad","afraid","sure",
    "certain","doubt","trust","believe","remember","forget","understand","realize","decide","choose",
    "prefer","accept","refuse","agree","disagree","promise","expect","suppose","imagine","wonder","guess",
    "assume","consider","hear","smell","taste","touch","sense","speak","talk","question","reply",
    "whisper","shout","write","draw","paint","sing","dance","help","serve","carry","lift","push","drop",
    "throw","catch","kick","hit","open","close","lock","unlock","enter","leave","arrive","depart",
    "travel","journey","walk","jump","climb","swim","fly","drive","ride","sail","navigate","create",
    "design","form","shape","cut","break","fix","repair","clean","wash","dry","wipe","sweep","mop",
    "polish","decorate","arrange","organize","prepare","bake","boil","fry","mix","stir","pour","wear",
    "remove","replace","exchange","sell","pay","spend","save","borrow","lend","owe","earn","win","lose",
    "continue","stop","pause","wait","hurry","rush","slow","quick","fast","rapid","swift","speed","pace",
    "rate","degree","level","stage","phase","part","piece","section","portion","share","percent","half",
    "double","triple","quarter","total","sum","average","mean","median","range","scale","measure","weight",
    "length","width","height","depth","size","color","pattern","style","figure","image","photo","video",
    "film","movie","song","music","sound","voice","tone","note","beat","rhythm","melody","harmony","chord",
    "instrument","piano","guitar","drum","violin","flute","trumpet","organ","band","orchestra","concert",
    "art","sculpture","statue","painting","drawing","camera","screen","monitor","computer","phone","email",
    "message","letter","card","sign","label","tag","mark","symbol","signal","flag","banner","poster","ad",
    "news","report","article","story","novel","poem","script","magazine","journal","document","file",
    "record","data","information","fact","detail","example","issue","topic","subject","theme","idea",
    "concept","notion","theory","principle","rule","law","standard","method","technique","approach",
    "strategy","plan","program","project","task","goal","target","objective","purpose","cause","effect",
    "result","outcome","consequence","impact","influence","power","strength","force","energy","vigor",
    "health","fitness","system","structure","network","web","chain","link","connection","relationship",
    "association","interaction","communication","discussion","conversation","dialogue","interview","speech",
    "presentation","lecture","seminar","workshop","training","education","learning","teaching","instruction",
    "guidance","advice","counsel","consultation","coaching","management","leadership","direction","control",
    "command","authority","responsibility","duty","obligation","commitment","effort","attempt","endeavor",
    "enterprise","venture","industry","corporation","organization","institution","agency","bureau",
    "department","division","headquarters","facility","equipment","tool","device","machine","engine","motor",
    "pump","valve","switch","button","lever","wire","cable","cord","rope","hook","clasp","latch","hinge",
    "bearing","material","substance","stuff","thing","object","item","product","goods","merchandise",
    "commodity","resource","supply","stock","inventory","warehouse","storage","container","package","box",
    "crate","barrel","jar","sack","pouch","envelope","parcel","shipment","cargo","freight","load","mass",
    "volume","capacity","space","area","region","zone","district","sector","territory","boundary","border",
    "limit","edge","margin","rim","brim","surface","exterior","inside","interior","middle","center","core",
    "heart","midst","bottom","base","foundation","basis","ground","step","grade","rank","position","status",
    "role","function","able","ability","aboard","aboriginal","abort","abortion","abound","abroad","abruptly",
    "absence","absolute","absorb","abstract","absurd","accent","access","accident","acclaim","accompany",
    "accomplish","accord","account","accuracy","accurate","accuse","achieve","acknowledge","acquire","across",
    "action","active","activist","activity","actor","actress","actual","acute","adapter","add","addict",
    "addition","adequate","adjust","adjustment","administration","admire","admission","admit","adolescent",
    "adopt","adult","advance","advanced","advantage","adventure","adverse","advocate","aerial","affair",
    "affect","afford","agenda","agent","aggressive","ago","agreement","agricultural","ahead","aid","aim",
    "air","aircraft","airline","album","alcohol","alert","alien","align","alike","alive","alleged","alliance",
    "allow","ally","almost","alone","along","aloof","already","alter","alternative","although","amateur",
    "amazing","ambassador","ambiguous","ambition","ambitious","amendment","america","american","amid","among",
    "amount","amuse","analysis","analyst","analyze","ancient","angle","angry","ankle","announce","annoying",
    "annual","another","anticipate","anxious","anybody","anymore","anyone","anything","anyway","anywhere",
    "apart","apartment","apparent","apparently","appeal","appear","appearance","apple","application","apply",
    "appoint","appointment","appreciate","appropriate","approval","approve","approximate","architect","argue",
    "argument","arise","arm","armed","army","array","arrest","arrival","arrow","artificial","artist","artistic",
    "asian","aside","asleep","aspect","assault","assert","assess","assessment","asset","assign","assignment",
    "assist","assistance","assistant","associate","assumption","assure","athlete","athletic","atmosphere",
    "attach","attack","attend","attention","attitude","attorney","attract","attractive","attribute","audience",
    "author","authorize","auto","automatic","available","avoid","award","aware","awareness","awful","awkward",
    "axis","backup","bad","badge","badly","balance","balcony","ball","ballet","balloon","ballot","banana",
    "bang","bar","barely","bargain","barrier","baseball","basic","basically","basin","basket","basketball",
    "bathroom","battery","battle","beach","bean",
];

// ─── Simple LCG PRNG ───
#[derive(Clone)]
pub struct SimpleRng {
    state: u64,
}

impl SimpleRng {
    pub fn new(seed: u64) -> Self {
        Self { state: seed }
    }

    pub fn next(&mut self) -> u64 {
        self.state = self.state.wrapping_mul(6364136223846793005).wrapping_add(1442695040888963407);
        self.state
    }

    pub fn next_usize(&mut self, max: usize) -> usize {
        (self.next() % max as u64) as usize
    }
}
