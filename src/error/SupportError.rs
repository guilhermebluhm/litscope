pub enum ErrorSupport{
    FalhaGeralProcessarRequisicaoHttp(String),
    FalhaAoMontarQueryParam(String),
    FalhaAoProcessarPublicationInfo(String),
}