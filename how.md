- explain how 6502 does pipeliing. [ref](http://users.telenet.be/kim1-6502/6502/proman.html)

1. DIRECT ADDRESSING
    
2. 
    INDEXED ADDRESSIN
     Absolute indexed address is absolute addressing with an index
     register added to the absolute address. 
     Zero-Paged Indexed address is zero-paged addressing with an index
     register added to the absolute address. 
3.  INDIRECT ADDRESSING
          In solving a certain class of problems, it is sometimes necessary
     to have an address which is a truly computed value, not just a base
     address with some type of offset, but a value which is calculated or
     sometimes obtained as a group of addresses.  In order to implement
     this type of indexing or addressing, the use of indirect addressing
     has been introduced.

#### __Why do we have Indexed Addressing?__  
As has been developed in many of the previous examples, an index register has primary values as a modifier and as a counter.  As a modifier to a base address operation, it allows the accessing of contiguous groups of data by simple modification of the index.