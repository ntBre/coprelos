from openff.toolkit.topology import ImproperDict, ValenceDict


def label_molecules(ff, top):
    tmp = ff.label_molecules(top)
    ret = []
    for t in tmp:
        r = {}
        for k, v in t.items():
            if isinstance(v, (ValenceDict, ImproperDict)):
                v = v.store
            r[k] = v
        ret.append(r)
    return ret
