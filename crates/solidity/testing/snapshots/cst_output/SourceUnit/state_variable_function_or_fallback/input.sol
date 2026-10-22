contract A {
    // State variables: an identifier followed by `;` or `=` is the variable name.
    function() a;
    function() external b;
    function() internal c;
    function() payable d;
    function() external e = f;
    function() external onlyOwner;

    // Unnamed (fallback) functions: there is no such identifier.
    function() external;
    function() external { }
    function() payable onlyOwner { }
    function() onlyOwner(1);
}
