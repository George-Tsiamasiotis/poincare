use nc::NcData;

mod common;

#[test]
fn test_nc_data_creation() -> Result<(), netcdf::Error> {
    let path = &common::phony_netcdf_path()?;
    let nd_data = NcData::open(path.into());

    assert!(nd_data.is_ok());

    std::fs::remove_file(path).expect("Failed to delete temp phony.nc");
    Ok(())
}
