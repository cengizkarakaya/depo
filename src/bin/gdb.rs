use lib::{alt_cizgi, ust_cizgi};
use rand::prelude::*;

fn maybe_pick<'a, T>(slice: &'a [T], rng: &mut impl Rng, prob: f64) -> Option<&'a T> {
    if rng.random_bool(prob) {
        slice.choose(rng)
    } else {
        None
    }
}

fn main() {
    ust_cizgi();

    let hair_styles = [
    "shoulder-length layered hair",
    "medium length wavy hair",
    "medium length straight hair",
    "medium bob with bangs hair",
    "medium shag hair",
    "medium wolf cut hair",
    "butterfly cut hair",
    "jellyfish cut hair",
    "hime cut hair",
    "face-framing layers hair",
    "curtain bangs + medium layers hair",
    "wispy bangs + medium length hair",
    "blunt medium cut hair",
    "flipped out ends hair",
    "long straight hair",
    "long silky straight hair",
    "long layered hair",
    "long feathered layers hair",
    "long shag hair",
    "long wolf cut hair",
    "long curly hair",
    "long loose curls hair",
    "long tight curls hair",
    "long spiral curls hair",
    "long beach waves hair",
    "long mermaid waves hair",
    "long soft waves hair",
    "long voluminous hair",
    "long flowing hair",
    "long center part hair",
    "long side part hair",
    "long curtain bangs hair",
    "long wispy bangs hair",
    "long blunt bangs hair",
    "long micro bangs hair",
    "long baby bangs hair",
    "long side-swept bangs hair",
];

    let hair_colors = [
        "black hair",
        "jet black hair",
        "raven black hair",
        "off-black hair",
        "dark brown hair",
        "chocolate brown hair",
        "chestnut brown hair",
        "medium brown hair",
        "light brown hair",
        "caramel brown hair",
        "ash brown hair",
        "mousy brown hair",
        "brunette hair",
        "blonde hair",
        "platinum blonde hair",
        "ash blonde hair",
        "honey blonde hair",
        "strawberry blonde hair",
        "dirty blonde hair",
        "golden blonde hair",
        "sandy blonde hair",
        "red hair",
        "auburn hair",
        "ginger hair",
        "copper hair",
        "fiery red hair",
        "mahogany hair",
        "wine red hair",
        "grey hair",
        "silver hair",
        "gray hair",
        "salt and pepper hair",
        "white hair",
        "snowy white hair",
        "platinum silver hair",
        "ash blonde hair",
        "ash gray hair",
        "beige blonde hair",
        "buttery blonde hair",
        "caramel highlights hair",
        "balayage hair",
        "ombre brown hair",
        "sun-kissed blonde hair",
        "toffee brown hair",
        "espresso brown hair",
        "cinnamon brown hair",
        "rose brown hair",
        "burgundy hair",
        "cherry red hair",
        "strawberry blonde hair",
        "copper red hair",
        "auburn red hair",
        "pink hair",
        "bubblegum pink hair",
        "pastel pink hair",
        "hot pink hair",
        "cotton candy pink hair",
        "magenta hair",
        "purple hair",
        "lavender hair",
        "lilac hair",
        "violet hair",
        "deep purple hair",
        "indigo hair",
        "blue hair",
        "sky blue hair",
        "electric blue hair",
        "turquoise hair",
        "teal hair",
        "mint green hair",
        "emerald green hair",
        "lime green hair",
        "neon green hair",
        "silver hair",
        "chrome silver hair",
        "holographic hair",
        "multicolored hair",
        "rainbow hair",
        "split color hair",
        "white and pastel pink hair",
        "lavender silver hair",
    ];

    let hair_other = [
        "vibrant hair",
        "soft hair",
        "muted hair",
        "saturated hair",
        "shiny hair",
        "silky hair",
        "matte hair",
        "two-tone hair",
        "gradient hair",
        "dyed hair",
        "pastel hair",
    ];

    let hair_textures = [
    "silky smooth hair",
    "glossy hair",
    "shiny reflective hair",
    "matte hair",
    "fluffy hair",
    "soft voluminous hair",
    "coarse rough hair",
    "frizzy hair",
    "wispy hair",
    "feathery hair",
    "cotton candy hair",
    "razor-sharp hair",
    "split ends hair",
    "windswept hair",
    "tousled messy hair",
];

let hair_volume = [
    "voluminous hair",
    "thick hair",
    "thin hair",
    "fine hair",
    "airy hair",
    "light hair",
    "bouncy curls hair",
    "flowing hair",
    "floating hair",
    "wind-blown hair",
    "gravity-defying hair",
    "layered volume hair",
    "flat hair",
    "limp hair",
    "big hair",
    "voluminous roots hair",
];

let hair_bangs = [
    "blunt bangs hair",
    "side-swept bangs hair",
    "curtain bangs hair",
    "wispy bangs hair",
    "micro bangs hair",
    "baby bangs hair",
    "see-through bangs hair",
    "choppy bangs hair",
    "asymmetrical bangs hair",
    "hime cut bangs hair",
    "fringe covering one eye hair",
    "hair covering eyes partially hair",
    "twin drills hair",
    "spiral curls",
];

let hair_light = [
    "hair highlights",
    "rim lighting on hair",
    "god rays through hair",
    "iridescent hair sheen",
    "metallic hair shine",
    "holographic highlights on hair",
    "soft hair glow",
    "dramatic hair shadows",
    "sun-kissed highlights on hair",
    "low-key hair lighting",
];

let hair_accessories = [
    "hair ribbons",
    "hair clips",
    "hair barrettes",
    "hair scrunchies",
    "bows in hair",
    "flowers in hair",
    "hair beads",
    "hair jewelry",
    "headband with hair",
    "hair chains",
    "metallic threads in hair",
    "crown braid with hair gems",
    "kanzashi hair ornaments",
    "hair pins",
    "ribbons woven through hair",
    "hair veil",
    "sheer fabric in hair",
];

let hair_conditions = [
    "wet hair dripping water",
    "sweaty hair strands sticking to face",
    "messy bedhead hair",
    "windswept tousled hair",
    "hair flowing underwater in water",
    "hair with snowflakes caught in strands",
    "ash-covered dirty hair post-apocalyptic style",
    "singed burnt hair tips damaged ends",
    "perfectly styled salon fresh hair",
    "overgrown roots showing dyed hair regrowth",
];

let hair_extra = [
    "boho loose strands hair",
    "cyberpunk neon streaks hair",
    "fantasy ethereal glowing hair",
    "vintage pin-up curls hair",
    "grunge messy layered hair",
    "kawaii bouncy cute hair",
    "gothic straight sleek hair",
    "beachy salt-sprayed wavy hair",
    "razor sharp bob hair",
    "undercut long top hair",
];

let hair_buns = [
    "messy bun hair",
    "high messy bun hair",
    "low messy bun hair",
    "sleek bun hair",
    "ballerina bun hair",
    "top knot hair",
    "space buns hair",
    "double buns hair",
    "low bun hair",
    "high bun with tendrils hair",
    "chignon hair",
    "low chignon hair",
    "french twist hair",
    "elegant updo hair",
    "braided updo hair",
    "sock bun hair",
];

let hair_ponytais = [
    "high ponytail hair",
    "sleek high ponytail hair",
    "messy high ponytail hair",
    "low ponytail hair",
    "side ponytail hair",
    "braided ponytail hair",
    "fishtail ponytail hair",
    "bubble ponytail hair",
    "high ponytail with bangs hair",
    "ponytail with scrunchie hair",
];

let hair_braids = [
    "french braid hair",
    "dutch braid hair",
    "fishtail braid hair",
    "waterfall braid hair",
    "boxer braids hair",
    "cornrows hair",
    "box braids hair",
    "fulani braids hair",
    "goddess braids hair",
    "bohemian braids hair",
    "braided crown hair",
    "braided headband hair",
    "milkmaid braids hair",
    "single braid hair",
    "twin braids hair",
    "pigtail braids hair",
    "rope braid hair",
    "rope twist ponytail hair",
];

let hair_curly = [
    "natural afro hair",
    "big voluminous afro hair",
    "tapered afro hair",
    "frohawk hair",
    "twist-out hair",
    "braid-out hair",
    "wash-and-go curls hair",
    "defined curls hair",
    "ringlet curls hair",
    "spiral perm curls hair",
    "bantu knots hair",
    "flat twists hair",
];

let hair_otherstyles = [
    "half-up half-down hair",
    "half-up top knot hair",
    "space buns half-up hair",
    "curtain bangs hair",
    "blunt bangs hair",
    "wispy fringe hair",
    "side-swept hair",
    "slicked-back hair",
    "wet-look hair",
    "messy tousled hair",
    "bedhead hair",
    "windswept hair",
    "boho waves hair",
    "glamour curls hair",
    "voluminous blowout hair",
    "flipped ends straight hair",
];

let mut rng = rand::rng();

    // zorunlu gruplar
    let x = hair_styles.choose(&mut rng).unwrap();
    let a = hair_colors.choose(&mut rng).unwrap();
    let b = hair_other.choose(&mut rng).unwrap();
    let c = hair_textures.choose(&mut rng).unwrap();
    let d = hair_volume.choose(&mut rng).unwrap();
    let f = hair_light.choose(&mut rng).unwrap();
    let g = hair_accessories.choose(&mut rng).unwrap();

    // opsiyonel gruplar (seçilme olasılığı %50)
    let e = maybe_pick(&hair_bangs, &mut rng, 0.5);
    let h = maybe_pick(&hair_conditions, &mut rng, 0.5);
    let i = maybe_pick(&hair_extra, &mut rng, 0.5);
    let j = maybe_pick(&hair_buns, &mut rng, 0.5);
    let k = maybe_pick(&hair_ponytais, &mut rng, 0.5);
    let l = maybe_pick(&hair_braids, &mut rng, 0.5);
    let m = maybe_pick(&hair_curly, &mut rng, 0.5);
    let n = maybe_pick(&hair_otherstyles, &mut rng, 0.5);

    // tüm parçaları topla
    let mut parts: Vec<String> = vec![
        x.to_string(),
        a.to_string(),
        b.to_string(),
        c.to_string(),
        d.to_string(),
        f.to_string(),
        g.to_string(),
    ];

    for opt in [e, h, i, j, k, l, m, n] {
        if let Some(v) = opt {
            parts.push(v.to_string());
        }
    }

    let result = parts.join(", ") + ".";
    println!("{}\n", result);

    // --- Face Shape ---
    let face_shapes = [
        "oval face shape, soft jawline, high cheekbones",
        "heart-shaped face, wide forehead, narrow pointed chin",
        "diamond face shape, prominent cheekbones, narrow forehead and chin",
        "round face with soft features, full cheeks",
        "square jawline, defined angular face",
        "oblong face, longer vertical proportions, balanced features",
        "slightly elongated oval face, gentle contours",
    ];

    // --- Eyes ---
    let eye_features = [
        "striking blue eyes, almond-shaped, long lashes",
        "deep green eyes, slightly hooded, intense gaze",
        "hazel eyes with golden flecks, round and expressive",
        "light grey-blue eyes, upturned outer corners",
        "warm brown eyes, deep-set, thick natural lashes",
        "icy blue eyes, wide-set, doe-eyed look",
        "emerald green eyes, cat-like shape",
        "stormy grey eyes, monolids subtle European variation",
    ];

    // --- Nose ---
    let nose_types = [
        "straight narrow nose, refined bridge",
        "small upturned button nose, delicate",
        "aquiline nose with gentle curve, aristocratic look",
        "straight Greek nose, strong defined bridge",
        "slightly wider nose with soft tip, natural European",
        "petite straight nose, subtle nostrils",
        "Roman nose profile, prominent but elegant",
    ];

    // --- Lips ---
    let lips = [
        "full plump lips, defined Cupid's bow",
        "medium heart-shaped lips, natural pink tone",
        "thin elegant lips, subtle definition",
        "softly pouty lips, slightly asymmetrical smile",
        "bow-shaped lips with natural volume",
        "wide full lips, soft corners",
    ];

    // --- Additional Details ---
    let face_details = [
        "high prominent cheekbones, sculpted look",
        "soft rounded cheeks, youthful appearance",
        "faint freckles across nose and cheeks",
        "smooth porcelain skin, subtle rosy undertones",
        "defined jawline with slight dimple on chin",
        "slightly wide-set eyes, innocent expression",
        "narrow face with sharp elegant features",
        "gentle double chin hint, natural realism",
        "prominent brow ridge, strong expressive brows",
        "soft under-eye area, no heavy bags",
    ];

    // --- Her gruptan 1 seçim ---
    let shape = face_shapes.choose(&mut rng).unwrap();
    let eyes = eye_features.choose(&mut rng).unwrap();
    let nose = nose_types.choose(&mut rng).unwrap();
    let mouth = lips.choose(&mut rng).unwrap();
    let detail = face_details.choose(&mut rng).unwrap();

    let result = format!("{}, {}, {}, {}, {}.", shape, eyes, nose, mouth, detail);
    println!("{}", result);

    alt_cizgi();
}
