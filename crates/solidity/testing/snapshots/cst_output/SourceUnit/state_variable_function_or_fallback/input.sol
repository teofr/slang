contract A {
    // State variables: an identifier followed by `;` or `=` is the variable name.
    function() a;
    function() external b;
    function() internal c;
    function() payable d;
    function() external e = f;
    function() external onlyOwner; // a variable named `onlyOwner`, not a modifier

    // Unnamed (fallback) functions: no identifier is followed by `;` or `=`.
    function() external; // no identifier: a function without a body
    function() external { } // no identifier: a function with a body
    function() payable onlyOwner { } // followed by `{`: a modifier
    function() onlyOwner(1); // followed by `(`: a modifier with arguments, and no body
    function() x external; // followed by `external`: a modifier, and no body
}
