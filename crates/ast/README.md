# AST

This crate contains all of the code related to representing PHP and PXP code.

## Overview

There are 2 main structures for representing code:
* `Statement`
* `Expression`

Both of these are represented with 3 fields:
* `kind` - represents the type of node.
* `span` - the location of the node in the source text.
* `comments` - any comments attached to the node.

Despite the name of the crate, the nodes actually form a _"concrete syntax tree"_ and not an abstract one. This means that every single piece of information is stored, including the positions of punctuation, keywords, etc.