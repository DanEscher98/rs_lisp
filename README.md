# rs_lisp

An easy implementation following "Build your Own Lisp"

```mermaid
flowchart TB
    v("values") --> g("global_st")
    v --> p("parse")
    v --> e("eval")
    g --> p
    g --> e
```
