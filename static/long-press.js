(function() {
    var btn = document.getElementById('long-press-btn');
    var form = document.getElementById('scan-form');
    if (!btn || !form) return;
    var pressTimer = null;

    function startPress(e) {
        if (pressTimer) return;

        btn.classList.add('pressing');

        pressTimer = setTimeout(function() {
            form.submit();
        }, 1000);
    }

    function cancelPress(e) {
        btn.classList.remove('pressing');
        if (pressTimer) {
            clearTimeout(pressTimer);
            pressTimer = null;
        }
    }

    // Mouse events
    btn.addEventListener('mousedown', startPress);
    btn.addEventListener('mouseup', cancelPress);
    btn.addEventListener('mouseleave', cancelPress);

    // Touch events
    btn.addEventListener('touchstart', function(e) {
        e.preventDefault();
        startPress(e);
    }, { passive: false });
    btn.addEventListener('touchend', cancelPress);
    btn.addEventListener('touchcancel', cancelPress);
})();
