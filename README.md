# Hypno

Chess engine on an early stage. 

---

# Movegen

Almost fully done. 

**Features**

Before generating, creates 2 Vec<Move>, for checks and xrays. 

When not in check, performs common searching for each piece in every possible direction; if a piece pinned, looks for moves within pinned line.

When king is in check, then skip the generation, and searches for pieces join a check line, eat piece and all king moves.

Uses move flags and board man flags, second to show passant eat and castle. Move flags are for multiple purposes, make_move speed up, pin and check manipulations, queening.

*ToDo:*

-3-fold

-Movegen adaptation for search mode

*Done*

-Successful testing with depth 6 for:
Starting, starting with no pawns, pins, castle ling & short, passant, pinned passant, queening.

-Updated board and piece representation for easier usage and slight boost in speed.