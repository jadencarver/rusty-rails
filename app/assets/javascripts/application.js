module.exports = class Application {
  constructor(jQuery) {
    jQuery(this.ready.bind(this));
    jQuery(document).on('click', '#tessellation rect', function(click) {
      click.target.classList.toggle("on");
    });
  }
  ready() {
    var resume = jQuery.ajax('/resume', { dataType: 'xml' });
    if (jQuery('#tessellation').length) {
      this.cycleTimer = setInterval(this.cycle, 5000);
      this.cycle();
      setTimeout(function() {
        clearInterval(this.cycleTimer);
        jQuery("#tessellation *").each(function(i,el) {
          el.classList.add('destructing');
        });
        setTimeout(function() {
          resume.done(function(data) {
            history.pushState({}, "Resume", "/resume");
            var body = data.querySelector('body');
            body.classList.add('transition');
            document.body.outerHTML = body.outerHTML;
            setTimeout(function() { document.body.classList.remove('transition'); }, 10);
          });
        }, 4000);
      }.bind(this), 5000);
    }
  }
  cycle() {
    jQuery('#tessellation rect').each(function(i,e) {
      var neighbors = [];
      if (e.classList.contains('tessellation-sq')) {
        neighbors.push(e.nextElementSibling);
        neighbors.push(e.nextElementSibling ? e.nextElementSibling.nextElementSibling : null);
        neighbors.push(e.previousElementSibling ? e.previousElementSibling.previousElementSibling : null);
        var prev = e;
        for(var j=1;j<=25;j++) {
          prev = prev ? prev.previousElementSibling : null;
          if (j === 25) neighbors.push(prev);
        }
      } else if (e.classList.contains('tessellation-rt')) {
        neighbors.push(e.previousElementSibling);
        neighbors.push(e.nextElementSibling);
        neighbors.push(e.nextElementSibling ? e.nextElementSibling.nextElementSibling : null);
        var prev = e;
        for(var j=1;j<=23;j++) {
          prev = prev ? prev.previousElementSibling : null;
          if (j === 23) neighbors.push(prev);
        }
      } else if (e.classList.contains('tessellation-bt')) {
        neighbors.push(e.previousElementSibling);
        neighbors.push(e.previousElementSibling ? e.previousElementSibling.previousElementSibling : null);
        var next = e;
        for(var j=1;j<=25;j++) {
          next = next ? next.nextElementSibling : null;
          if (j === 23 || j == 25) neighbors.push(next)
        }
      }
      e.setAttribute('data-neighbors', neighbors.filter(function(i) { return i && i.classList.contains('on') }).length);
    });
    jQuery('#tessellation rect').each(function(i,e) {
      setTimeout(function() {
        var neighbors = parseInt(e.getAttribute('data-neighbors'));
        if (e.classList.contains('on') && (neighbors < 2 || neighbors > 3)) {
          e.classList.remove('on');
        } else if (neighbors == 3 || Math.random() > 0.9) {
          e.classList.add('on');
        }
        e.classList.add('highlight');
        setTimeout(function() {
          e.classList.remove('highlight')
        }, 200);
      }, i * 25);
    });
  }
}
