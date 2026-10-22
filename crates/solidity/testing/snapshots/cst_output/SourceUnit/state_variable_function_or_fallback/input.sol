contract A {
    // State variables: an identifier followed by `;` or `=` is the variable name.
    function() a; // state variable `a`, not a function with an `a` modifier
    function() external b; // state variable `b`, not a function with a `b` modifier
    function() internal c; // state variable `c`, not a function with a `c` modifier
    function() payable d; // state variable `d`, not a function with a `d` modifier
    function() external e = f; // state variable `e` with a value
    function() external onlyOwner; // state variable `onlyOwner`, not a function with an `onlyOwner` modifier

    // Unnamed (fallback) functions: no identifier is followed by `;` or `=`.
    function() external; // function without a body, not a state variable
    function() external { } // function with a body, not a state variable
    function() payable onlyOwner { } // function with a body, not a state variable
    function() onlyOwner(1); // function with an `onlyOwner(1)` modifier, not a state variable
    function() x external; // function with an `x` modifier, not a state variable
}
