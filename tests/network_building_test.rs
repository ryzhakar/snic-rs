use snic::network::take_elements_uniformly;

#[test]
fn test_take_elements_uniformly() {
    assert_eq!(
        take_elements_uniformly(20, 2, 10).collect::<Vec<u32>>(),
        vec![10, 20],
    );
    assert_eq!(
        take_elements_uniformly(3, 2, 100).collect::<Vec<u32>>(),
        vec![100, 101],
    );
    assert_eq!(
        take_elements_uniformly(1001, 5, 1).collect::<Vec<u32>>(),
        vec![1, 201, 401, 601, 801],
    );
}
