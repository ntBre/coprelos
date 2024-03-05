def get_k(param):
    if not isinstance(param.k, list):
        k = [param.k]
    else:
        k = param.k
    return [v.magnitude for v in k]
