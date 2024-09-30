use smartcore::ensemble::random_forest_classifier::RandomForestClassifier;
use smartcore::metrics::accuracy;
use smartcore::linalg::basic::matrix::DenseMatrix;
use std::collections::HashMap;
use crate::ml_output;
use smartcore::model_selection::train_test_split;
use std::process::Command;

fn label_encode(s: &str, map: &mut HashMap<String, usize>) -> f64 {
    let s = if s.is_empty() { "unknown" } else { s };
    if !map.contains_key(s) {
        let len = map.len();
        map.insert(s.to_string(), len);
    }
    *map.get(s).unwrap() as f64
}

fn parse_or_default(value: &str) -> f64 {
    if value.is_empty() {
        2.0
    } else {
        value.parse().unwrap_or_default()
    }
}

fn normalize(features: &mut Vec<Vec<f64>>) {
    let n_features = features[0].len();
    let n_samples = features.len();

    for i in 0..n_features {
        let mean = features.iter().map(|x| x[i]).sum::<f64>() / n_samples as f64;
        let variance = features
            .iter()
            .map(|x| (x[i] - mean).powi(2))
            .sum::<f64>() / n_samples as f64;
        let stddev = variance.sqrt();

        for j in 0..n_samples {
            features[j][i] = (features[j][i] - mean) / stddev;
        }
    }
}

pub fn carregar_dados() -> (Vec<Vec<f64>>, Vec<usize>) {
    let mut features = Vec::new();
    let mut labels = Vec::new();

    // Mapas para codificação
    let mut ro_build_tags_map = HashMap::new();
    let mut ro_build_user_map = HashMap::new();
    let mut ro_build_host_map = HashMap::new();
    let mut ro_build_flavor_map = HashMap::new();
    let mut ro_product_model_map = HashMap::new();
    let mut ro_product_manufacturer_map = HashMap::new();
    let mut ro_product_device_map = HashMap::new();
    let mut ro_hardware_map = HashMap::new();
    let mut ro_boot_mode_map = HashMap::new();
    let mut init_svc_qemud_map = HashMap::new();
    let mut init_svc_qemu_props_map = HashMap::new();
    let mut ro_product_manufacturer_1_map = HashMap::new();
    let mut ro_product_model_1_map = HashMap::new();
    let mut ro_product_device_1_map = HashMap::new();
    let mut ro_product_model_2_map = HashMap::new();
    let mut ro_build_version_release_map = HashMap::new();
    let mut ro_build_characteristics_map = HashMap::new();
    let mut ro_build_type_map = HashMap::new();
    let mut ro_build_version_security_patch_map = HashMap::new();

    // Itera sobre os dados do array
    for record in ml_output::DATA.iter() {
        features.push(vec![
            label_encode(record[1], &mut ro_build_tags_map),
            parse_or_default(record[2]),
            parse_or_default(record[3]),
            label_encode(record[4], &mut ro_build_user_map),
            label_encode(record[5], &mut ro_build_host_map),
            label_encode(record[6], &mut ro_build_flavor_map),
            parse_or_default(record[7]),
            label_encode(record[8], &mut ro_product_model_map),
            label_encode(record[9], &mut ro_product_manufacturer_map),
            label_encode(record[10], &mut ro_product_device_map),
            label_encode(record[11], &mut ro_hardware_map),
            parse_or_default(record[12]),
            label_encode(record[13], &mut ro_boot_mode_map),
            label_encode(record[14], &mut init_svc_qemud_map),
            label_encode(record[15], &mut init_svc_qemu_props_map),
            label_encode(record[16], &mut ro_product_manufacturer_1_map),
            label_encode(record[17], &mut ro_product_model_1_map),
            parse_or_default(record[18]),
            label_encode(record[19], &mut ro_product_device_1_map),
            label_encode(record[20], &mut ro_product_model_2_map),
            label_encode(record[21], &mut ro_build_version_release_map),
            label_encode(record[22], &mut ro_build_characteristics_map),
            label_encode(record[23], &mut ro_build_type_map),
            label_encode(record[24], &mut ro_build_version_security_patch_map),
        ]);

        labels.push(if record[0] == "Sim" {
            1
        } else {
            0
        });
    }

    (features, labels)
}

pub fn treinar_modelo(
    mut features: Vec<Vec<f64>>,
    labels: Vec<usize>,
) -> (RandomForestClassifier<f64, usize, DenseMatrix<f64>, Vec<usize>>, f64) {
    normalize(&mut features);
    let x = DenseMatrix::from_2d_vec(&features);
    let y = labels;

    let (x_train, x_test, y_train, y_test) =
        train_test_split(&x, &y, 0.3, true, Some(42));

    let modelo = RandomForestClassifier::fit(&x_train, &y_train, Default::default()).unwrap();

    let y_pred = modelo.predict(&x_test).unwrap();
    let acc = accuracy(&y_test, &y_pred);

    (modelo, acc)
}

pub fn prever_ambiente(
    dados: Vec<String>,
    model: &RandomForestClassifier<f64, usize, DenseMatrix<f64>, Vec<usize>>,
) -> bool {
    let mut ro_build_tags_map = HashMap::new();
    let mut ro_build_user_map = HashMap::new();
    let mut ro_build_host_map = HashMap::new();
    let mut ro_build_flavor_map = HashMap::new();
    let mut ro_product_model_map = HashMap::new();
    let mut ro_product_manufacturer_map = HashMap::new();
    let mut ro_product_device_map = HashMap::new();
    let mut ro_hardware_map = HashMap::new();
    let mut ro_boot_mode_map = HashMap::new();
    let mut init_svc_qemud_map = HashMap::new();
    let mut init_svc_qemu_props_map = HashMap::new();
    let mut ro_product_manufacturer_1_map = HashMap::new();
    let mut ro_product_model_1_map = HashMap::new();
    let mut ro_product_device_1_map = HashMap::new();
    let mut ro_product_model_2_map = HashMap::new();
    let mut ro_build_version_release_map = HashMap::new();
    let mut ro_build_characteristics_map = HashMap::new();
    let mut ro_build_type_map = HashMap::new();
    let mut ro_build_version_security_patch_map = HashMap::new();

    let features: Vec<f64> = vec![
        label_encode(&dados[0], &mut ro_build_tags_map),
        parse_or_default(&dados[1]),
        parse_or_default(&dados[2]),
        label_encode(&dados[3], &mut ro_build_user_map),
        label_encode(&dados[4], &mut ro_build_host_map),
        label_encode(&dados[5], &mut ro_build_flavor_map),
        parse_or_default(&dados[6]),
        label_encode(&dados[7], &mut ro_product_model_map),
        label_encode(&dados[8], &mut ro_product_manufacturer_map),
        label_encode(&dados[9], &mut ro_product_device_map),
        label_encode(&dados[10], &mut ro_hardware_map),
        parse_or_default(&dados[11]),
        label_encode(&dados[12], &mut ro_boot_mode_map),
        label_encode(&dados[13], &mut init_svc_qemud_map),
        label_encode(&dados[14], &mut init_svc_qemu_props_map),
        label_encode(&dados[15], &mut ro_product_manufacturer_1_map),
        label_encode(&dados[16], &mut ro_product_model_1_map),
        parse_or_default(&dados[17]),
        label_encode(&dados[18], &mut ro_product_device_1_map),
        label_encode(&dados[19], &mut ro_product_model_2_map),
        label_encode(&dados[20], &mut ro_build_version_release_map),
        label_encode(&dados[21], &mut ro_build_characteristics_map),
        label_encode(&dados[22], &mut ro_build_type_map),
        label_encode(&dados[23], &mut ro_build_version_security_patch_map),
    ];

    let x = DenseMatrix::from_2d_vec(&vec![features]);
    let y_pred = model.predict(&x).unwrap();

    y_pred[0] == 1
}


pub fn get_android_properties() -> Vec<String> {
    let properties = [
        "ro.build.tags",//1
        "ro.debuggable",//2
        "ro.secure",//3
        "ro.build.user",//4
        "ro.build.host",//5
        "ro.build.flavor",//6
        "ro.kernel.qemu",//7
        "ro.product.model",//8
        "ro.product.manufacturer",//9
        "ro.product.device",//10
        "ro.hardware",//11
        "ro.kernel.android.qemud",//12
        "ro.boot.mode",//13
        "init.svc.qemud",//14
        "init.svc.qemu-props",//15
        "ro.product.manufacturer",//16
        "ro.product.model",//17
        "ro.kernel.qemu.gles",//18
        "ro.product.device",//19
        "ro.product.model",//20
        "ro.build.version.release",//21
        "ro.build.characteristics",//22
        "ro.build.type",//23
        "ro.build.version.security_patch",//24
    ];

    properties.iter().map(|prop| {
        match Command::new("getprop").arg(prop).output() {
            Ok(output) if output.status.success() => {
                String::from_utf8_lossy(&output.stdout).trim().to_string()
            },
            _ => String::from("unknown")
        }
    }).collect()
}
