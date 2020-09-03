let heatMapControl = (function() {
  let defaultMap = null;
  let heatmaps = [];

  return {
    setDefault: function(heatmap) {
      defaultMap = heatmap;
    },
    attach: function(heatmap) {
      heatmaps.push(heatmap);
    },
    select: function(selected) {
      let selectedOne = false;
      for(let i=0; i<heatmaps.length; i++) {
        if (heatmaps[i] == selected) {
          selectedOne = true
          heatmaps[i].style.display = 'inherit';
        } else {
          heatmaps[i].style.display = 'none';
        }
      }
      if (!selectedOne && defaultMap) {
        defaultMap.style.display = 'inherit';
      } else {
        defaultMap.style.display = 'none';
      }
    }
  }
})();

window.onload = function(){
  let ctl = document.getElementById('hm-ctls');
  if (!ctl) { return; }
  ctl.style.display = 'inherit';

  const shortcodes = [
    'p-lab',
    'p-con',
    'p-ldm',
    'p-grn',
    'p-brx',
    'p-snp',
    'g-wmn',
    'g-man',
    'g-oth',
  ];

  let all = document.getElementById('hm-all');
  if (all) {
    heatMapControl.setDefault(all);
  }

  for(let i=0; i<shortcodes.length; i++){
    let hmId = 'hm-' + shortcodes[i];
    let ctlId = 'hm-ctl-' + shortcodes[i];
    let hm = document.getElementById(hmId);
    let ctl = document.getElementById(ctlId);
    if (!hm || !ctl) { continue; }
    heatMapControl.attach(hm);
    ctl.onclick = function() { heatMapControl.select(hm); }
  }
};
