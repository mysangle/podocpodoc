//
//  FriendViewModel.swift
//  podocpodoc
//
//  Created by soonhyung-imac on 2/20/17.
//  Copyright © 2017 twentyhours. All rights reserved.
//

import UIKit
import RxSwift
import RxDataSources

class FriendViewModel {
    
    func getUsers() -> Observable<[SectionModel<String, Friend>]> {
        return Observable.create { (observer) -> Disposable in
            
            let friends = [Friend(label: "label1", address: "address1"),
                           Friend(label: "label2", address: "address2"),
                           Friend(label: "label3", address: "address3")]
            let section = [SectionModel(model: "", items: friends)]
            observer.onNext(section)
            observer.onCompleted()
            
            return Disposables.create()
        }
    }
}
