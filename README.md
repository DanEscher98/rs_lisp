# rs_lisp

An easy implementation following "Build your Own Lisp"

```mermaid
flowchart TB
    R("run") --> P("parser")
    P --> E("eval")
    P --> G("global_st")
    P --> V("tokens")
    E --> V
    E --> G
    G --> V
```
