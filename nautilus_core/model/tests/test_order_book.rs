// -------------------------------------------------------------------------------------------------
//  Copyright (C) 2015-2024 Nautech Systems Pty Ltd. All rights reserved.
//  https://nautechsystems.io
//
//  Licensed under the GNU Lesser General Public License Version 3.0 (the "License");
//  You may not use this file except in compliance with the License.
//  You may obtain a copy of the License at https://www.gnu.org/licenses/lgpl-3.0.en.html
//
//  Unless required by applicable law or agreed to in writing, software
//  distributed under the License is distributed on an "AS IS" BASIS,
//  WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
//  See the License for the specific language governing permissions and
//  limitations under the License.
// -------------------------------------------------------------------------------------------------

use nautilus_model::{enums::BookType, identifiers::InstrumentId, orderbook::book::OrderBook};
use nautilus_test_kit::{
    common::{get_project_testdata_path, get_testdata_large_checksums_filepath},
    files::ensure_file_exists_or_download_http,
};
use rstest::*;

#[rstest]
pub fn test_order_book_databento_mbo_nasdaq() {
    let testdata = get_project_testdata_path();
    let checksums = get_testdata_large_checksums_filepath();
    let filename = "databento_mbo_xnas_itch.csv";
    let filepath = testdata.join("large").join(filename);
    let url = "https://hist.databento.com/v0/dataset/sample/download/xnas.itch/mbo";
    ensure_file_exists_or_download_http(&filepath, url, Some(&checksums)).unwrap();

    let instrument_id = InstrumentId::from("AAPL.XNAS");
    let _ = OrderBook::new(instrument_id, BookType::L3_MBO);

    // assert_eq!(book.best_bid_price().unwrap(), price);
    // assert_eq!(book.best_ask_price().unwrap(), price);
    // assert_eq!(book.best_bid_size().unwrap(), size);
    // assert_eq!(book.best_ask_size().unwrap(), size);
}
