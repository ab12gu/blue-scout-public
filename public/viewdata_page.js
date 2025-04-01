const TABLE_FILTER_OPTIONS_BASE = {
  base_path: "tablefilter/",
  sticky_headers: true,
  rows_counter: true,
  flt_css_class: "input input-primary",
  div_checklist_css_class: "card bg-base-100 border-primary",
  checklist_css_class: "m-1",
  clear_filter_text: "None",
  enable_checklist_reset_filter: false,
  themes: [
    {
      name: "transparent",
    },
  ],
};

const TABLE_FILTER_OPTIONS_FULL = {
  ...TABLE_FILTER_OPTIONS_BASE,
  __COL_FILTER_DEFS__,
};
const TABLE_FILTER_OPTIONS_REDUCED = {
  ...TABLE_FILTER_OPTIONS_BASE,
  __COL_FILTER_DEFS_REDUCED__,
};

function preventMinusSign(event) {
  if (event.key === "-") {
    event.preventDefault();
  }
}

document.addEventListener("_leptos_hydrated", function (event) {
  // const tf = new TableFilter(
  //   document.getElementById("scouting_data_table"),
  //   document.getElementById("fullDataCheckbox").checked
  //     ? TABLE_FILTER_OPTIONS_FULL
  //     : TABLE_FILTER_OPTIONS_REDUCED,
  // );
  // window.tf = tf;
  // function reloadTableFilter() {
  //   window.tf.destroy();
  //   setTimeout(function () {
  //     const tf = new TableFilter(
  //       document.getElementById("scouting_data_table"),
  //       document.getElementById("fullDataCheckbox").checked
  //         ? TABLE_FILTER_OPTIONS_FULL
  //         : TABLE_FILTER_OPTIONS_REDUCED,
  //     );
  //     window.tf = tf;
  //     tf.init();
  //   }, 250);
  // }
  // window.reloadTableFilter = reloadTableFilter;
  // reloadTableFilter();
});
