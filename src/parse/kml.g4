grammar kml;

// Lexer rules
COMMENT       : '#' ~[\r\n]* -> skip ;

IDENTIFIER    : [a-zA-Z_][a-zA-Z_0-9]* ;

WS            : [ \t\r\n]+ -> skip ;

// Parser rules
kernelModel   : (eventDef | COMMENT)* kernelDef EOF ;

eventDef      : 'event' IDENTIFIER '{' eventBody '}' ;
kernelDef     : 'kernel' '{' kernelBody '}' ;

eventBody     : (eventAction | COMMENT)* ;
kernelBody    : (schedulerDef | eventsDef | COMMENT)* ;

eventAction   : 'shutdown' 
              | 'newtask' 
              | 'exit' 
              | 'sched' 
              | 'stop' ;

schedulerDef  : 'scheduler' '=' schedulerType ;
eventsDef     : 'events' '=' '[' (IDENTIFIER (',' IDENTIFIER)*)? ']' ;

schedulerType : 'fifo' ;
