(function() {
"use strict";

const OR_CHOICE = Object.freeze({
    NO:   0,
    YES:  1,
    BOTH: 2,
});

function apply_filters(filters) {
    const min_cog_lv = filters.level_range[0];
    const max_cog_lv = filters.level_range[1];

    const min_toon_count = filters.toon_range[0];
    const max_toon_count = filters.toon_range[1];

    const ok_filter_names = ["lured", "v2", "org"];

    for (let i = 1; i <= 12; ++i) {
        const i_elems = document.getElementsByClassName(`level-${i}`);
        for (const i_elem of i_elems) {
            i_elem.classList.remove("hidden");
        }
    }

    for (let i = 1; i <= 12; ++i) {
        const i_elems = document.getElementsByClassName(`${i}-toons`);
        for (const i_elem of i_elems) {
            i_elem.classList.remove("hidden");
        }
    }

    for (const filter_name of ok_filter_names) {
        const class_name = filter_name === "org" ? "org-row" : filter_name;

        const true_elems = document.getElementsByClassName(class_name);
        for (const true_elem of true_elems) {
            true_elem.classList.remove("hidden");
        }

        const false_elems =
            document.getElementsByClassName(`not-${class_name}`);
        for (const false_elem of false_elems) {
            false_elem.classList.remove("hidden");
        }
    }

    /* \\\\\\\\\\\\\\\\\\\\\\\\\\\\\\\\\\\\\\\\\\\\\\\\\\\\\\\\\ */

    for (let i = 1; i <= 12; ++i) {
        const i_elems = document.getElementsByClassName(`level-${i}`);
        for (const i_elem of i_elems) {
            if (i < min_cog_lv || i > max_cog_lv) {
                i_elem.classList.add("hidden");
            }
        }
    }

    for (let i = 1; i <= 12; ++i) {
        const i_elems = document.getElementsByClassName(`${i}-toons`);
        for (const i_elem of i_elems) {
            if (i < min_toon_count || i > max_toon_count) {
                i_elem.classList.add("hidden");
            }
        }
    }

    for (const filter_name of ok_filter_names) {
        const class_name = filter_name === "org" ? "org-row" : filter_name;

        const true_elems = document.getElementsByClassName(class_name);
        for (const true_elem of true_elems) {
            if (filters[filter_name] === OR_CHOICE.NO) {
                true_elem.classList.add("hidden");
            }
        }

        const false_elems =
            document.getElementsByClassName("not-" + class_name);
        for (const false_elem of false_elems) {
            if (filters[filter_name] === OR_CHOICE.YES) {
                false_elem.classList.add("hidden");
            }
        }
    }
}

window.onload = function() {
    const filters = {
        level_range: [1, 12],
        lured:       OR_CHOICE.BOTH,
        v2:          OR_CHOICE.BOTH,
        toon_range:  [1, 4],
        org:         OR_CHOICE.BOTH,
    };
    const level_range_low_input = document.getElementById("level-range-low");
    const level_range_high_input = document.getElementById("level-range-high");
    const lured_both_input = document.getElementById("lured-both");
    const lured_yes_input = document.getElementById("lured-yes");
    const lured_no_input = document.getElementById("lured-no");
    const v2_both_input = document.getElementById("v2-both");
    const v2_yes_input = document.getElementById("v2-yes");
    const v2_no_input = document.getElementById("v2-no");
    const toon_range_low_input = document.getElementById("toons-low");
    const toon_range_high_input = document.getElementById("toons-high");
    const org_both_input = document.getElementById("org-both");
    const org_yes_input = document.getElementById("org-yes");
    const org_no_input = document.getElementById("org-no");

    function register_range(filter_name, low_input, high_input) {
        low_input.onchange = e => {
            const new_val = +e.target.value;
            if (new_val > filters[filter_name][1]) {
                e.target.value = filters[filter_name][0];
            } else {
                filters[filter_name][0] = new_val;
            }
            apply_filters(filters);
        };
        high_input.onchange = e => {
            const new_val = +e.target.value;
            if (new_val < filters[filter_name][0]) {
                e.target.value = filters[filter_name][1];
            } else {
                filters[filter_name][1] = new_val;
            }
            apply_filters(filters);
        };

        low_input.value = filters[filter_name][0];
        high_input.value = filters[filter_name][1];
    }
    function register_or_choice(filter_name, both_input, yes_input, no_input) {
        both_input.onchange = e => {
            if (e.target.value === "on") {
                filters[filter_name] = OR_CHOICE.BOTH;
                apply_filters(filters);
            }
        };
        yes_input.onchange = e => {
            if (e.target.value === "on") {
                filters[filter_name] = OR_CHOICE.YES;
                apply_filters(filters);
            }
        };
        no_input.onchange = e => {
            if (e.target.value === "on") {
                filters[filter_name] = OR_CHOICE.NO;
                apply_filters(filters);
            }
        };

        both_input.checked = true;
        yes_input.checked = false;
        no_input.checked = false;
    }

    register_range(
        "level_range",
        level_range_low_input,
        level_range_high_input
    );
    register_or_choice(
        "lured",
        lured_both_input,
        lured_yes_input,
        lured_no_input
    );
    register_or_choice(
        "v2",
        v2_both_input,
        v2_yes_input,
        v2_no_input
    );
    register_range(
        "toon_range",
        toon_range_low_input,
        toon_range_high_input
    );
    register_or_choice(
        "org",
        org_both_input,
        org_yes_input,
        org_no_input
    );
};

})();
